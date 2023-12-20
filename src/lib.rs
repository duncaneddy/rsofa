#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Provide default constructors for SOFA types
impl Default for iauASTROM {
    fn default() -> iauASTROM {
        iauASTROM {
            pmt: 0.0,
            eb: [0.0, 0.0, 0.0],
            eh: [0.0, 0.0, 0.0],
            em: 0.0,
            v: [0.0, 0.0, 0.0],
            bm1: 0.0,
            bpn: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            along: 0.0,
            phi: 0.0, 
            xpl: 0.0,
            ypl: 0.0,
            sphi: 0.0,
            cphi: 0.0,
            diurab: 0.0,
            eral: 0.0,
            refa: 0.0,
            refb: 0.0
        }
    }
}

impl Default for iauLDBODY {
    fn default() -> iauLDBODY {
        iauLDBODY {
            bm: 0.0,
            dl: 0.0, 
            pv: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn test_default_iauASTROM() {
        let astrom = iauASTROM::default();
        
        assert_eq!(astrom.pmt, 0.0);
        assert_eq!(astrom.eb, [0.0, 0.0, 0.0]);
        assert_eq!(astrom.eh, [0.0, 0.0, 0.0]);
        assert_eq!(astrom.em, 0.0);
        assert_eq!(astrom.v, [0.0, 0.0, 0.0]);
        assert_eq!(astrom.bm1, 0.0);
        assert_eq!(astrom.bpn, [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
        assert_eq!(astrom.along, 0.0);
        assert_eq!(astrom.phi, 0.0);
        assert_eq!(astrom.xpl, 0.0);
        assert_eq!(astrom.ypl, 0.0);
        assert_eq!(astrom.sphi, 0.0);
        assert_eq!(astrom.cphi, 0.0);
        assert_eq!(astrom.diurab, 0.0);
        assert_eq!(astrom.eral, 0.0);
        assert_eq!(astrom.refa, 0.0);
        assert_eq!(astrom.refb, 0.0);
    }
    
    #[test]
    fn test_default_iauLDBODY() {
        let ldbody = iauLDBODY::default();
        
        assert_eq!(ldbody.bm, 0.0);
        assert_eq!(ldbody.dl, 0.0);
        assert_eq!(ldbody.pv, [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
    }
}