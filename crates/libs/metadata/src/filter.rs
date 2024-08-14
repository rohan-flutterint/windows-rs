// TODO: should move to bindgen

use super::*;
use std::collections::btree_map::*;

type TypeMapImp = BTreeMap<&'static str, BTreeMap<&'static str, &'static [Item]>>;

pub struct TypeMap( pub TypeMapImp);

impl TypeMap {
    pub fn new(reader: &'static Reader, include: &[&str], exclude: &[&str], add_dependencies: bool ) -> Self {
        let filter = Filter::new(include, exclude);
        let mut map = BTreeMap::new();

        for (namespace, items) in &reader.items {
            if filter.includes_namespace(namespace) {
                for (name, items) in items {
                    if filter.includes_type_name(namespace, name) {
                        add_type_name(reader, &mut map, namespace, name, Some(items), add_dependencies);
                    }
                }
            }
        }

        Self( map)
    }
}

// Returns whether the value was newly inserted.
fn add_type_name(
    reader: &'static Reader,
    map: &mut TypeMapImp,
    namespace: &'static str,
    name: &'static str,
    items: Option<&'static [Item]>,
    add_dependencies: bool,
) -> bool {
    if let Entry::Vacant(entry) = map.entry(namespace).or_default().entry(name) {
        let items = items.unwrap_or_else(|| {
            reader
                .items
                .get(namespace)
                .expect("Namespace not found")
                .get(name)
                .expect("Name not found")
        });
        entry.insert(items);

        if add_dependencies {
        for item in items {
            match item {
                Item::Type(def) => {
                    add_type_dependencies(reader, map, &Type::TypeDef(*def, vec![]))
                }
                Item::Const(field) => {
                    add_type_dependencies(reader, map, &field.ty(None).to_const_type())
                }
                Item::Fn(method, _namespace) => {
                    let signature = method.signature(&[]);
                    add_type_dependencies(reader, map, &signature.return_type);
                    signature
                        .params
                        .iter()
                        .for_each(|ty| add_type_dependencies(reader, map, ty));
                }
            }
        }
        }


        true
    } else {
        false
    }
}

// Add this type (via `add_type_name`) and its dependencies.
fn add_type_dependency(reader: &'static Reader, map: &mut TypeMapImp, ty: &Type) {
    if let Type::TypeDef(def, generics) = ty.to_underlying_type() {
        let namespace = def.namespace();
        if namespace.is_empty() {
            def.fields().for_each(|field|add_type_dependency(reader, map, &field.ty(Some(def))));
        } else {
            add_type_name(reader, map, namespace, def.name(), None, true);
            generics.iter().for_each(|ty|add_type_dependency(reader, map, ty));
        }
        
    }
}

// Add this type's dependencies.
fn add_type_dependencies(reader: &'static Reader, map: &mut TypeMapImp, ty: &Type) {
    if let Type::TypeDef(def, generics) = ty.to_underlying_type() {
        generics.iter().for_each(|ty| add_type_dependency(reader, map, ty));
        def.fields()
            .for_each(|field| add_type_dependency(reader, map, &field.ty(Some(def))));
    }
}


#[derive(Default)]
pub struct Filter(pub Vec<(String, bool)>);

impl Filter {
    pub fn new(include: &[&str], exclude: &[&str]) -> Self {
        let mut rules = vec![];

        for include in include {
            rules.push((include.to_string(), true));
        }

        for exclude in exclude {
            rules.push((exclude.to_string(), false));
        }

        rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        Self(rules)
    }

    pub fn includes_namespace(&self, namespace: &str) -> bool {
        if self.0.is_empty() {
            return true;
        }

        for rule in &self.0 {
            if rule.1 {
                // include
                if rule.0.starts_with(namespace) {
                    return true;
                }
                if namespace.starts_with(&rule.0) {
                    return true;
                }
            } else {
                // exclude
                if namespace.starts_with(&rule.0) {
                    return false;
                }
            }
        }

        false
    }

    pub fn includes_type_name(&self, namespace: &str, name: &str) -> bool {
        if self.0.is_empty() {
            return true;
        }

        for rule in &self.0 {
            if match_type_name(&rule.0, namespace, name) {
                return rule.1;
            }
        }

        false
    }
}

fn match_type_name(rule: &str, namespace: &str, name: &str) -> bool {
    if rule.len() <= namespace.len() {
        return namespace.starts_with(rule);
    }

    if !rule.starts_with(namespace) {
        return false;
    }

    if rule.as_bytes()[namespace.len()] != b'.' {
        return false;
    }

    name == &rule[namespace.len() + 1..]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn includes_type_name(filter: &Filter, full_name: &'static str) -> bool {
        let type_name = crate::TypeName::parse(full_name);
        filter.includes_type_name(type_name.namespace(), type_name.name())
    }

    #[test]
    fn test_namespace() {
        let include = ["N1.N2"];
        let exclude = ["N1.N2.N3"];
        let f = Filter::new(&include, &exclude);

        assert!(f.includes_namespace("N1"));
        assert!(f.includes_namespace("N1.N2"));
        assert!(f.includes_namespace("N1.N2.N4"));

        assert!(!f.includes_namespace("N1.N2.N3"));
        assert!(!f.includes_namespace("N1.N2.N3.N4"));
    }

    #[test]
    fn test_simple() {
        let include = ["N1", "N3", "N3.N4.N5"];
        let exclude = ["N2", "N3.N4"];
        let f = Filter::new(&include, &exclude);

        assert!(!includes_type_name(&f, "NN.T"));

        assert!(includes_type_name(&f, "N1.T"));
        assert!(includes_type_name(&f, "N3.T"));

        assert!(!includes_type_name(&f, "N2.T"));
        assert!(!includes_type_name(&f, "N3.N4.T"));

        assert!(includes_type_name(&f, "N3.N4.N5.T"));
    }

    #[test]
    fn filter_excludes_same_length() {
        let include = ["N.N1", "N.N2"];
        let exclude = ["N.N3", "N.N4"];
        let f = Filter::new(&include, &exclude);

        assert!(includes_type_name(&f, "N.N1.T"));
        assert!(includes_type_name(&f, "N.N2.T"));

        assert!(!includes_type_name(&f, "N.N3.T"));
        assert!(!includes_type_name(&f, "N.N4.T"));
    }

    #[test]
    fn filter_exclude_include_precedence() {
        let include = ["N.T"];
        let exclude = ["N.T"];
        let f = Filter::new(&include, &exclude);

        assert!(!includes_type_name(&f, "N.T"));
    }
}
