use windows::core::*;
use windows::{Foundation::*, Win32::System::Com::*, Win32::System::Com::Marshal::*};

#[implement(IStringable)]
struct Stringable;

impl IStringable_Impl for Stringable {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
            let s: IStringable = Stringable.into();
    assert!(s.cast::<IAgileObject>().is_ok());
    //assert!(s.cast::<IMarshal>().is_ok());

    CoIncrementMTAUsage()?;

    // TODO: since this works perhaps we don't need an explicit IMarshal implementation any longer.

    let stream = CoMarshalInterThreadInterfaceInStream(&IStringable::IID, &s)?;
    let copy: IStringable = CoUnmarshalInterface(&stream)?;

    assert_eq!(copy.ToString()?, "Stringable");
    assert_eq!(s, copy);

    Ok(())
    }
}
