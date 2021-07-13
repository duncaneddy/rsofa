#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[allow(improper_ctypes)]
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
#[macro_use]
extern crate approx;

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use crate::*;

    #[test]
    fn test_a2af() {
        let mut idmsf: [i32; 4] = [0, 0, 0, 0];
        // let mut s:char;
        let mut s: i8 = 0;

        unsafe {
            iauA2af(4, 2.345, &mut s, &mut idmsf[0]);
        }

        assert_eq!(idmsf[0], 134);
        assert_eq!(idmsf[1], 21);
        assert_eq!(idmsf[2], 30);
        assert_eq!(idmsf[3], 9706);
    }

    #[test]
    fn test_a2tf() {
        let mut idmsf: [i32; 4] = [0, 0, 0, 0];
        // let mut s:char;
        let mut s:i8 = 0;

        unsafe {
            iauA2tf(4, -3.01234, &mut s, &mut idmsf[0]);
        }

        assert_eq!(idmsf[0], 11);
        assert_eq!(idmsf[1], 30);
        assert_eq!(idmsf[2], 22);
        assert_eq!(idmsf[3], 6484);
    }

    #[test]
    fn test_ab() {

        let mut pnat = [-0.76321968546737951, -0.60869453983060384, -0.21676408580639883];
        let mut v    = [2.1044018893653786e-5, -8.9108923304429319e-5, -3.8633714797716569e-5];
        let s    = 0.99980921395708788;
        let bm1  = 0.99999999506209258;
        let mut ppr: [f64; 3] = [0.0, 0.0, 0.0];

        unsafe {
            iauAb(&mut pnat[0], &mut v[0], s, bm1, &mut ppr[0]);
        }

        abs_diff_eq!(ppr[0], -0.7631631094219556269, epsilon=1.0e-12);
        abs_diff_eq!(ppr[1], -0.6087553082505590832, epsilon=1.0e-12);
        abs_diff_eq!(ppr[2], -0.2167926269368471279, epsilon=1.0e-12);
            
    }

    #[test]
    fn test_ae2hd() {

        let a = 5.5;
        let e = 1.1;
        let p = 0.7;
        let mut h = 0.0;
        let mut d = 0.0;

        unsafe {
            iauAe2hd(a, e, p, &mut h, &mut d);
        }

        abs_diff_eq!(h, 0.5933291115507309663, epsilon=1e-14);
        abs_diff_eq!(d, 0.9613934761647817620, epsilon=1e-14);

    }

    #[test]
    fn test_af2a() {

        let mut a = 0.0;
        let s = '-' as i8;

        let j;
        
        unsafe {
            j = iauAf2a(s, 45, 13, 27.2, &mut a);
        }
        
        abs_diff_eq!(a, -0.7893115794313644842, epsilon=1e-12);
        assert_eq!(j, 0);

    }

    #[test]
    fn test_anp() {
        unsafe {
            abs_diff_eq!(iauAnp(-0.1), 6.183185307179586477, epsilon=1e-12);
        }
    }

    #[test]
    fn test_anpm() {
        unsafe {
            abs_diff_eq!(iauAnpm(-4.0), 2.283185307179586477, epsilon=1e-12);
        }
    }

    #[test]
    fn test_apcg() {
        let date1 = 2456165.5;
        let date2 = 0.401182685;
        let mut ebpv = [[0.901310875, -0.417402664, -0.180982288],
                     [0.00742727954, 0.0140507459, 0.00609045792]];
        let mut ehp = [0.903358544, -0.415395237, -0.180084014];
        let mut astrom = iauASTROM {..Default::default()};

        unsafe {
            iauApcg(date1, date2, &mut ebpv[0], &mut ehp[0], &mut astrom);
        }

        abs_diff_eq!(astrom.pmt, 12.65133794027378508, epsilon=1e-11);
        abs_diff_eq!(astrom.eb[0], 0.901310875, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[1], -0.417402664, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[2], -0.180982288, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[0], 0.8940025429324143045, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[1], -0.4110930268679817955, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[2], -0.1782189004872870264, epsilon=1e-12);
        abs_diff_eq!(astrom.em, 1.010465295811013146, epsilon=1e-12);
        abs_diff_eq!(astrom.v[0], 0.4289638913597693554e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[1], 0.8115034051581320575e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[2], 0.3517555136380563427e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.bm1, 0.9999999951686012981, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][0], 1.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[1][0], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[2][0], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[0][1], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[1][1], 1.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[2][1], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[0][2], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[1][2], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[2][2], 1.0, epsilon=0.0);
    }

    #[test]
    fn test_apcg13() {
        let date1 = 2456165.5;
        let date2 = 0.401182685;
        let mut astrom = iauASTROM {..Default::default()};

        unsafe {
            iauApcg13(date1, date2, &mut astrom);
        }

        abs_diff_eq!(astrom.pmt, 12.65133794027378508, epsilon=1e-11);
        abs_diff_eq!(astrom.eb[0], 0.9013108747340644755, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[1], -0.4174026640406119957, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[2], -0.1809822877867817771, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[0], 0.8940025429255499549, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[1], -0.4110930268331896318, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[2], -0.1782189006019749850, epsilon=1e-12);
        abs_diff_eq!(astrom.em, 1.010465295964664178, epsilon=1e-12);
        abs_diff_eq!(astrom.v[0], 0.4289638912941341125e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[1], 0.8115034032405042132e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[2], 0.3517555135536470279e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.bm1, 0.9999999951686013142, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][0], 1.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[1][0], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[2][0], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[0][1], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[1][1], 1.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[2][1], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[0][2], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[1][2], 0.0, epsilon=0.0);
        abs_diff_eq!(astrom.bpn[2][2], 1.0, epsilon=0.0);
    }

    #[test]
    fn test_apci() {
        let date1 = 2456165.5;
        let date2 = 0.401182685;
        let mut ebpv = [[0.901310875, -0.417402664, -0.180982288],  
                    [0.00742727954,  0.0140507459,  0.00609045792]];
        let mut ehp = [0.903358544, -0.415395237, -0.180084014];
        let x =  0.0013122272;
        let y = -2.92808623e-5;
        let s =  3.05749468e-8;
        let mut astrom = iauASTROM {..Default::default()};

        unsafe {
            iauApci(date1, date2, &mut ebpv[0], &mut ehp[0], x, y, s, &mut astrom);
        }

        abs_diff_eq!(astrom.pmt, 12.65133794027378508, epsilon=1e-11);
        abs_diff_eq!(astrom.eb[0], 0.901310875, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[1], -0.417402664, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[2], -0.180982288, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[0], 0.8940025429324143045, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[1], -0.4110930268679817955, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[2], -0.1782189004872870264, epsilon=1e-12);
        abs_diff_eq!(astrom.em, 1.010465295811013146, epsilon=1e-12);
        abs_diff_eq!(astrom.v[0], 0.4289638913597693554e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[1], 0.8115034051581320575e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[2], 0.3517555136380563427e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.bm1, 0.9999999951686012981, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][0], 0.9999991390295159156, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][0], 0.4978650072505016932e-7, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][0], 0.1312227200000000000e-2, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][1], -0.1136336653771609630e-7, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][1], 0.9999999995713154868, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][1], -0.2928086230000000000e-4, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][2], -0.1312227200895260194e-2, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][2], 0.2928082217872315680e-4, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][2], 0.9999991386008323373, epsilon=1e-12);
    }

    #[test]
    fn test_apci13() {
        let date1 = 2456165.5;
        let date2 = 0.401182685;
        let mut astrom = iauASTROM {..Default::default()};
        let mut eo = 0.0;

        unsafe {
            iauApci13(date1, date2, &mut astrom, &mut eo);
        }

        abs_diff_eq!(astrom.pmt, 12.65133794027378508, epsilon=1e-11);
        abs_diff_eq!(astrom.eb[0], 0.9013108747340644755, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[1], -0.4174026640406119957, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[2], -0.1809822877867817771, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[0], 0.8940025429255499549, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[1], -0.4110930268331896318, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[2], -0.1782189006019749850, epsilon=1e-12);
        abs_diff_eq!(astrom.em, 1.010465295964664178, epsilon=1e-12);
        abs_diff_eq!(astrom.v[0], 0.4289638912941341125e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[1], 0.8115034032405042132e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[2], 0.3517555135536470279e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.bm1, 0.9999999951686013142, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][0], 0.9999992060376761710, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][0], 0.4124244860106037157e-7, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][0], 0.1260128571051709670e-2, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][1], -0.1282291987222130690e-7, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][1], 0.9999999997456835325, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][1], -0.2255288829420524935e-4, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][2], -0.1260128571661374559e-2, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][2], 0.2255285422953395494e-4, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][2], 0.9999992057833604343, epsilon=1e-12);
        abs_diff_eq!(eo, -0.2900618712657375647e-2, epsilon=1e-12);
    }

    #[test]
    fn test_apco() {
        let date1 = 2456384.5;
        let date2 = 0.970031644;
        let mut ebpv = [[-0.974170438, -0.211520082, -0.0917583024],
                    [0.00364365824, -0.0154287319, -0.00668922024]];
        let mut ehp = [-0.973458265, -0.209215307, -0.0906996477];
        let x = 0.0013122272;
        let y = -2.92808623e-5;
        let s = 3.05749468e-8;
        let theta = 3.14540971;
        let elong = -0.527800806;
        let phi = -1.2345856;
        let hm = 2738.0;
        let xp = 2.47230737e-7;
        let yp = 1.82640464e-6;
        let sp = -3.01974337e-11;
        let refa = 0.000201418779;
        let refb = -2.36140831e-7;
        let mut astrom = iauASTROM {..Default::default()};

        unsafe {
            iauApco(date1, date2, &mut ebpv[0], &mut ehp[0], x, y, s,
                theta, elong, phi, hm, xp, yp, sp,
                refa, refb, &mut astrom);
        }
        

        abs_diff_eq!(astrom.pmt, 13.25248468622587269, epsilon=1e-11);
        abs_diff_eq!(astrom.eb[0], -0.9741827110630322720, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[1], -0.2115130190135344832, epsilon=1e-12);
        abs_diff_eq!(astrom.eb[2], -0.09179840186949532298, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[0], -0.9736425571689739035, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[1], -0.2092452125849330936, epsilon=1e-12);
        abs_diff_eq!(astrom.eh[2], -0.09075578152243272599, epsilon=1e-12);
        abs_diff_eq!(astrom.em, 0.9998233241709957653, epsilon=1e-12);
        abs_diff_eq!(astrom.v[0], 0.2078704992916728762e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[1], -0.8955360107151952319e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.v[2], -0.3863338994288951082e-4, epsilon=1e-16);
        abs_diff_eq!(astrom.bm1, 0.9999999950277561236, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][0], 0.9999991390295159156, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][0], 0.4978650072505016932e-7, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][0], 0.1312227200000000000e-2, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][1], -0.1136336653771609630e-7, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][1], 0.9999999995713154868, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][1], -0.2928086230000000000e-4, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[0][2], -0.1312227200895260194e-2, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[1][2], 0.2928082217872315680e-4, epsilon=1e-12);
        abs_diff_eq!(astrom.bpn[2][2], 0.9999991386008323373, epsilon=1e-12);
        abs_diff_eq!(astrom.along, -0.5278008060301974337, epsilon=1e-12);
        abs_diff_eq!(astrom.xpl, 0.1133427418174939329e-5, epsilon=1e-17);
        abs_diff_eq!(astrom.ypl, 0.1453347595745898629e-5, epsilon=1e-17);
        abs_diff_eq!(astrom.sphi, -0.9440115679003211329, epsilon=1e-12);
        abs_diff_eq!(astrom.cphi, 0.3299123514971474711, epsilon=1e-12);
        assert_eq!(astrom.diurab, 0.0);
        abs_diff_eq!(astrom.eral, 2.617608903969802566, epsilon=1e-12);
        abs_diff_eq!(astrom.refa, 0.2014187790000000000e-3, epsilon=1e-15);
        abs_diff_eq!(astrom.refb, -0.2361408310000000000e-6, epsilon=1e-18);
    }

    #[test]
    fn test_tpxes() {
        let ra = 1.3;
        let dec = 1.55;
        let raz = 2.3;
        let decz = 1.5;

        let mut j = 0;
        let mut xi = 0.0;
        let mut eta = 0.0;

        unsafe {
            j = iauTpxes(ra, dec, raz, decz, &mut xi, &mut eta);
        }

        abs_diff_eq!(xi, -0.01753200983236980595, epsilon=1e-15);
        abs_diff_eq!(eta, 0.05962940005778712891, epsilon=1e-15);
    }


    #[test]
    fn test_tpxev() {
        let ra = 1.3;
        let dec = 1.55;
        let raz = 2.3;
        let decz = 1.5;

        let mut v = 0.0;
        let mut vz = 0.0;

        let mut xi = 0.0;
        let mut eta = 0.0;

        unsafe {
            iauS2c(ra, dec, &mut v);
            iauS2c(raz, decz, &mut vz);
        }

        abs_diff_eq!(xi, -0.01753200983236980595, epsilon=1e-15);
        abs_diff_eq!(eta, 0.05962940005778712891, epsilon=1e-15);
    }

    #[test]
    fn test_tr() {
        let mut r = [[2.0, 3.0, 2.0],
                    [3.0, 2.0, 3.0],
                    [3.0, 4.0, 5.0]];
                    
        let mut rt = [[0.0; 3]; 3];

            
        unsafe {
            iauTr(&mut r[0], &mut rt[0]);
        }

       assert_eq!(rt[0][0], 2.0);
       assert_eq!(rt[0][1], 3.0);
       assert_eq!(rt[0][2], 3.0);

       assert_eq!(rt[1][0], 3.0);
       assert_eq!(rt[1][1], 2.0);
       assert_eq!(rt[1][2], 4.0);

       assert_eq!(rt[2][0], 2.0);
       assert_eq!(rt[2][1], 3.0);
       assert_eq!(rt[2][2], 5.0);
    }

    #[test]
    fn test_trxp() {
        let mut r = [[2.0, 3.0, 2.0],
                    [3.0, 2.0, 3.0],
                    [3.0, 4.0, 5.0]];
                    
        let mut pv = [0.2, 1.5, 0.1];

        let mut trp = [0.0; 3];

            
        unsafe {
            iauTrxp(&mut r[0], &mut pv[0], &mut trp[0]);
        }

        abs_diff_eq!(trp[0], 5.2, epsilon=1e-12);
        abs_diff_eq!(trp[1], 4.0, epsilon=1e-12);
        abs_diff_eq!(trp[2], 5.4, epsilon=1e-12);
    }


    #[test]
    fn test_trxpv() {
        let mut r = [[2.0, 3.0, 2.0],
                    [3.0, 2.0, 3.0],
                    [3.0, 4.0, 5.0]];
                    
        let mut pv = [[0.2, 1.5, 0.1],
                      [1.5, 0.2, 0.1]];

        let mut trpv = [[0.0; 3]; 3];

            
        unsafe {
            iauTrxpv(&mut r[0], &mut pv[0], &mut trpv[0]);
        }

       abs_diff_eq!(trpv[0][0], 5.2, epsilon=1e-12);
       abs_diff_eq!(trpv[0][1], 4.0, epsilon=1e-12);
       abs_diff_eq!(trpv[0][2], 5.4, epsilon=1e-12);

       abs_diff_eq!(trpv[1][0], 3.9, epsilon=1e-12);
       abs_diff_eq!(trpv[1][1], 5.3, epsilon=1e-12);
       abs_diff_eq!(trpv[1][2], 4.1, epsilon=1e-12);
    }

    #[test]
    fn test_tttai() {
        let mut a1 = 0.0;
        let mut a2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauTttai(2453750.5, 0.892482639, &mut a1, &mut a2);
        }

        abs_diff_eq!(a1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(a2, 0.892110139, epsilon=1e-12);
        assert_eq!(j, 0);

    }


    #[test]
    fn test_tttcg() {
        let mut g1 = 0.0;
        let mut g2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauTttcg(2453750.5, 0.892855139, &mut g1, &mut g2);
        }

        abs_diff_eq!(g1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(g2, 0.8924900312508587113, epsilon=1e-12);
        assert_eq!(j, 0);

    }

    #[test]
    fn test_tttdb() {
        let mut b1 = 0.0;
        let mut b2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauTttdb(2453750.5, 0.892855139, -0.000201, &mut b1, &mut b2);
        }

        abs_diff_eq!(b1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(b2, 0.8928551366736111111, epsilon=1e-12);
        assert_eq!(j, 0);

    }

    #[test]
    fn test_tttut1() {
        let mut u1 = 0.0;
        let mut u2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauTtut1(2453750.5, 0.892104561, 64.8499, &mut u1, &mut u2);
        }

        abs_diff_eq!(u1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(u2, 0.8921045614537037037, epsilon=1e-12);
        assert_eq!(j, 0);

    }


    #[test]
    fn test_ut1tai() {
        let mut a1 = 0.0;
        let mut a2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauUt1tai(2453750.5, 0.892104561, -32.6659, &mut a1, &mut a2);
        }

        abs_diff_eq!(a1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(a2, 0.8924826385462962963, epsilon=1e-12);
        assert_eq!(j, 0);

    }

    #[test]
    fn test_ut1tt() {
        let mut t1 = 0.0;
        let mut t2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauUt1utc(2453750.5, 0.892104561, 64.8499, &mut t1, &mut t2);
        }

        abs_diff_eq!(t1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(t2, 0.8928551385462962963, epsilon=1e-12);
        assert_eq!(j, 0);

    }

    #[test]
    fn test_ut1utc() {
        let mut u1 = 0.0;
        let mut u2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauUt1utc(2453750.5, 0.892104561, 0.3341, &mut u1, &mut u2);
        }

        abs_diff_eq!(u1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(u2, 0.8921006941018518519, epsilon=1e-12);
        assert_eq!(j, 0);

    }


    #[test]
    fn test_utctai() {
        let mut u1 = 0.0;
        let mut u2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauUtctai(2453750.5, 0.892100694, &mut u1, &mut u2);
        }

        abs_diff_eq!(u1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(u2, 0.8924826384444444444, epsilon=1e-12);
        assert_eq!(j, 0);

    }



    #[test]
    fn test_utcut1() {
        let mut u1 = 0.0;
        let mut u2 = 0.0;
        let mut j  = 0;

        unsafe {
            j = iauUtcut1(2453750.5, 0.892100694, 0.3341, &mut u1, &mut u2);
        }

        abs_diff_eq!(u1, 2453750.5, epsilon=1e-6);
        abs_diff_eq!(u2, 0.8921045608981481481, epsilon=1e-12);
        assert_eq!(j, 0);

    }

    #[test]
    fn test_xy06() {
        let mut x = 0.0;
        let mut y = 0.0;

        unsafe {
            iauXy06(2400000.5, 53736.0, &mut x, &mut y);
        }

        abs_diff_eq!(x,  0.5791308486706010975e-3, epsilon=1e-15);
        abs_diff_eq!(y,  0.4020579816732958141e-4, epsilon=1e-16);
    }


    #[test]
    fn test_xys00a() {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut s = 0.0;

        unsafe {
            iauXys00a(2400000.5, 53736.0, &mut x, &mut y, &mut s);
        }

        abs_diff_eq!(x,  0.5791308472168152904e-3, epsilon=1e-14);
        abs_diff_eq!(y,  0.4020595661591500259e-4, epsilon=1e-15);
        abs_diff_eq!(s, -0.1220040848471549623e-7, epsilon=1e-18);
    }

    #[test]
    fn test_xys00b() {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut s = 0.0;

        unsafe {
            iauXys00b(2400000.5, 53736.0, &mut x, &mut y, &mut s);
        }

        abs_diff_eq!(x,  0.5791301929950208873e-3, epsilon=1e-14);
        abs_diff_eq!(y,  0.4020553681373720832e-4, epsilon=1e-15);
        abs_diff_eq!(s, -0.1220027377285083189e-7, epsilon=1e-18);
    }

    #[test]
    fn test_xys06a() {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut s = 0.0;

        unsafe {
            iauXys06a(2400000.5, 53736.0, &mut x, &mut y, &mut s);
        }

        abs_diff_eq!(x,  0.5791308482835292617e-3, epsilon=1e-14);
        abs_diff_eq!(y,  0.4020580099454020310e-4, epsilon=1e-15);
        abs_diff_eq!(s, -0.1220032294164579896e-7, epsilon=1e-18);
    }

    #[test]
    fn test_zp(){
        let mut p = [0.3, 1.2, -2.5];

        unsafe {
            iauZp(&mut p[0]);
        }
    }

    #[test]
    fn test_pv() {
        let mut pv = [[0.3, 1.2, -2.5],
                      [-0.5, 3.1, 0.9]];

        unsafe {
            iauZpv(&mut pv[0]);
        }

        assert_eq!(pv[0][0], 0.0);
        assert_eq!(pv[0][1], 0.0);
        assert_eq!(pv[0][2], 0.0);

        assert_eq!(pv[1][0], 0.0);
        assert_eq!(pv[1][1], 0.0);
        assert_eq!(pv[1][2], 0.0);
    }

    #[test]
    fn test_zr() {
        let mut r = [[0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0],
                     [0.0, 0.0, 0.0]];

        unsafe {
            iauZr(&mut r[0]);
        }

        assert_eq!(r[0][0], 0.0);
        assert_eq!(r[1][0], 0.0);
        assert_eq!(r[2][0], 0.0);

        assert_eq!(r[0][1], 0.0);
        assert_eq!(r[1][1], 0.0);
        assert_eq!(r[2][1], 0.0);

        assert_eq!(r[0][2], 0.0);
        assert_eq!(r[1][2], 0.0);
        assert_eq!(r[2][2], 0.0);
    }
}