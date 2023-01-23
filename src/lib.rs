pub mod json;
pub use self::json::*;
pub use rjson_macro::Serializable;

#[cfg(test)]
mod tests {
    use crate::json::{JsonParser, Result};

    #[test]
    fn check_int() -> Result<()> {
        let r = JsonParser::parse("123")?.int()?;
        assert_eq!(r, 123);
        Ok(())
    }

    #[test]
    fn check_float() -> Result<()> {
        let r = JsonParser::parse("123.234")?.float()?;
        assert_eq!(r, 123.234);
        Ok(())
    }

    #[test]
    fn check_string() -> Result<()> {
        let r = JsonParser::parse("\"bruhmoment\"")?.string()?;
        assert_eq!(r, "bruhmoment");
        Ok(())
    }
    #[test]
    fn check_object() -> Result<()> {
        let json = "{ 
            \"starburst\":true,
            \"stream\":12345
        }";
        let r = JsonParser::parse(&json)?;
        assert!(r.is_object());
        let r = r.object()?;
        assert_eq!(r.get("starburst")?.bool()?, true);
        assert_eq!(r.get("stream")?.int()?, 12345);
        Ok(())
    }

    #[test]
    fn check_array() -> Result<()> {
        let json = "[123,\"bruh\",true,null]";
        let r = JsonParser::parse(&json)?;
        assert!(r.is_array());
        let r = r.array()?;
        assert_eq!(r.get(0)?.int()?, 123);
        assert_eq!(r.get(1)?.string()?, "bruh");
        assert_eq!(r.get(2)?.bool()?, true);
        assert!(r.get(3)?.is_null());
        Ok(())
    }
    #[test]
    fn check_array_compound() -> Result<()> {
        let json = "[{
            \"hi\":123
        },{
            \"hi\":456
        },{
            \"hi\":789,
            \"kirito\":false
        }]";
        let r = JsonParser::parse(&json)?;
        assert!(r.is_array());
        let r = r.array()?;
        assert_eq!(r.get(0)?.object()?.get("hi")?.int()?, 123);
        assert_eq!(r.get(1)?.object()?.get("hi")?.int()?, 456);
        assert_eq!(r.get(2)?.object()?.get("hi")?.int()?, 789);
        assert_eq!(r.get(2)?.object()?.get("kirito")?.bool()?, false);
        Ok(())
    }
    #[test]
    fn check_nest_array() -> Result<()> {
        let json = "[[0,1,2],[3,4,5],[6,7,8]]";
        let r = JsonParser::parse(&json)?;
        assert!(r.is_array());
        let r = r.array()?;
        let mut c = 0;
        for i in 0..r.len() {
            let t = r.get(i)?.array()?;
            for j in 0..t.len() {
                assert_eq!(t.get(j)?.int()?, c);
                c += 1;
            }
        }
        Ok(())
    }
}
