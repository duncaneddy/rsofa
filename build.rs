extern crate cc;
extern crate bindgen;

fn main() {
    // Generate Rust Bindings for C Library
    let bindings = bindgen::Builder::default()
        .header("./extern/sofa.h") // Separate header than the source header
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("./src/bindings.rs")
        .expect("Unable to save bindings");

    // Compile C library
    cc::Build::new()
        .include("./extern/src")
        .file("./extern/src/a2af.c")
        .file("./extern/src/a2tf.c")
        .file("./extern/src/ab.c")
        .file("./extern/src/ae2hd.c")
        .file("./extern/src/af2a.c")
        .file("./extern/src/anp.c")
        .file("./extern/src/anpm.c")
        .file("./extern/src/apcg.c")
        .file("./extern/src/apcg13.c")
        .file("./extern/src/apci.c")
        .file("./extern/src/apci13.c")
        .file("./extern/src/apco.c")
        .file("./extern/src/apco13.c")
        .file("./extern/src/apcs.c")
        .file("./extern/src/apcs13.c")
        .file("./extern/src/aper.c")
        .file("./extern/src/aper13.c")
        .file("./extern/src/apio.c")
        .file("./extern/src/apio13.c")
        .file("./extern/src/atci13.c")
        .file("./extern/src/atciq.c")
        .file("./extern/src/atciqn.c")
        .file("./extern/src/atciqz.c")
        .file("./extern/src/atco13.c")
        .file("./extern/src/atic13.c")
        .file("./extern/src/aticq.c")
        .file("./extern/src/aticqn.c")
        .file("./extern/src/atio13.c")
        .file("./extern/src/atioq.c")
        .file("./extern/src/atoc13.c")
        .file("./extern/src/atoi13.c")
        .file("./extern/src/atoiq.c")
        .file("./extern/src/bi00.c")
        .file("./extern/src/bp00.c")
        .file("./extern/src/bp06.c")
        .file("./extern/src/bpn2xy.c")
        .file("./extern/src/c2i00a.c")
        .file("./extern/src/c2i00b.c")
        .file("./extern/src/c2i06a.c")
        .file("./extern/src/c2ibpn.c")
        .file("./extern/src/c2ixy.c")
        .file("./extern/src/c2ixys.c")
        .file("./extern/src/c2s.c")
        .file("./extern/src/c2t00a.c")
        .file("./extern/src/c2t00b.c")
        .file("./extern/src/c2t06a.c")
        .file("./extern/src/c2tcio.c")
        .file("./extern/src/c2teqx.c")
        .file("./extern/src/c2tpe.c")
        .file("./extern/src/c2txy.c")
        .file("./extern/src/cal2jd.c")
        .file("./extern/src/cp.c")
        .file("./extern/src/cpv.c")
        .file("./extern/src/cr.c")
        .file("./extern/src/d2dtf.c")
        .file("./extern/src/d2tf.c")
        .file("./extern/src/dat.c")
        .file("./extern/src/dtdb.c")
        .file("./extern/src/dtf2d.c")
        .file("./extern/src/eceq06.c")
        .file("./extern/src/ecm06.c")
        .file("./extern/src/ee00.c")
        .file("./extern/src/ee00a.c")
        .file("./extern/src/ee00b.c")
        .file("./extern/src/ee06a.c")
        .file("./extern/src/eect00.c")
        .file("./extern/src/eform.c")
        .file("./extern/src/eo06a.c")
        .file("./extern/src/eors.c")
        .file("./extern/src/epb.c")
        .file("./extern/src/epb2jd.c")
        .file("./extern/src/epj.c")
        .file("./extern/src/epj2jd.c")
        .file("./extern/src/epv00.c")
        .file("./extern/src/eqec06.c")
        .file("./extern/src/eqeq94.c")
        .file("./extern/src/era00.c")
        .file("./extern/src/fad03.c")
        .file("./extern/src/fae03.c")
        .file("./extern/src/faf03.c")
        .file("./extern/src/faju03.c")
        .file("./extern/src/fal03.c")
        .file("./extern/src/falp03.c")
        .file("./extern/src/fama03.c")
        .file("./extern/src/fame03.c")
        .file("./extern/src/fane03.c")
        .file("./extern/src/faom03.c")
        .file("./extern/src/fapa03.c")
        .file("./extern/src/fasa03.c")
        .file("./extern/src/faur03.c")
        .file("./extern/src/fave03.c")
        .file("./extern/src/fk425.c")
        .file("./extern/src/fk45z.c")
        .file("./extern/src/fk524.c")
        .file("./extern/src/fk52h.c")
        .file("./extern/src/fk54z.c")
        .file("./extern/src/fk5hip.c")
        .file("./extern/src/fk5hz.c")
        .file("./extern/src/fw2m.c")
        .file("./extern/src/fw2xy.c")
        .file("./extern/src/g2icrs.c")
        .file("./extern/src/gc2gd.c")
        .file("./extern/src/gc2gde.c")
        .file("./extern/src/gd2gc.c")
        .file("./extern/src/gd2gce.c")
        .file("./extern/src/gmst00.c")
        .file("./extern/src/gmst06.c")
        .file("./extern/src/gmst82.c")
        .file("./extern/src/gst00a.c")
        .file("./extern/src/gst00b.c")
        .file("./extern/src/gst06.c")
        .file("./extern/src/gst06a.c")
        .file("./extern/src/gst94.c")
        .file("./extern/src/h2fk5.c")
        .file("./extern/src/hd2ae.c")
        .file("./extern/src/hd2pa.c")
        .file("./extern/src/hfk5z.c")
        .file("./extern/src/icrs2g.c")
        .file("./extern/src/ir.c")
        .file("./extern/src/jd2cal.c")
        .file("./extern/src/jdcalf.c")
        .file("./extern/src/ld.c")
        .file("./extern/src/ldn.c")
        .file("./extern/src/ldsun.c")
        .file("./extern/src/lteceq.c")
        .file("./extern/src/ltecm.c")
        .file("./extern/src/lteqec.c")
        .file("./extern/src/ltp.c")
        .file("./extern/src/ltpb.c")
        .file("./extern/src/ltpecl.c")
        .file("./extern/src/ltpequ.c")
        .file("./extern/src/num00a.c")
        .file("./extern/src/num00b.c")
        .file("./extern/src/num06a.c")
        .file("./extern/src/numat.c")
        .file("./extern/src/nut00a.c")
        .file("./extern/src/nut00b.c")
        .file("./extern/src/nut06a.c")
        .file("./extern/src/nut80.c")
        .file("./extern/src/nutm80.c")
        .file("./extern/src/obl06.c")
        .file("./extern/src/obl80.c")
        .file("./extern/src/p06e.c")
        .file("./extern/src/p2pv.c")
        .file("./extern/src/p2s.c")
        .file("./extern/src/pap.c")
        .file("./extern/src/pas.c")
        .file("./extern/src/pb06.c")
        .file("./extern/src/pdp.c")
        .file("./extern/src/pfw06.c")
        .file("./extern/src/plan94.c")
        .file("./extern/src/pm.c")
        .file("./extern/src/pmat00.c")
        .file("./extern/src/pmat06.c")
        .file("./extern/src/pmat76.c")
        .file("./extern/src/pmp.c")
        .file("./extern/src/pmpx.c")
        .file("./extern/src/pmsafe.c")
        .file("./extern/src/pn.c")
        .file("./extern/src/pn00.c")
        .file("./extern/src/pn00a.c")
        .file("./extern/src/pn00b.c")
        .file("./extern/src/pn06.c")
        .file("./extern/src/pn06a.c")
        .file("./extern/src/pnm00a.c")
        .file("./extern/src/pnm00b.c")
        .file("./extern/src/pnm06a.c")
        .file("./extern/src/pnm80.c")
        .file("./extern/src/pom00.c")
        .file("./extern/src/ppp.c")
        .file("./extern/src/ppsp.c")
        .file("./extern/src/pr00.c")
        .file("./extern/src/prec76.c")
        .file("./extern/src/pv2p.c")
        .file("./extern/src/pv2s.c")
        .file("./extern/src/pvdpv.c")
        .file("./extern/src/pvm.c")
        .file("./extern/src/pvmpv.c")
        .file("./extern/src/pvppv.c")
        .file("./extern/src/pvstar.c")
        .file("./extern/src/pvtob.c")
        .file("./extern/src/pvu.c")
        .file("./extern/src/pvup.c")
        .file("./extern/src/pvxpv.c")
        .file("./extern/src/pxp.c")
        .file("./extern/src/refco.c")
        .file("./extern/src/rm2v.c")
        .file("./extern/src/rv2m.c")
        .file("./extern/src/rx.c")
        .file("./extern/src/rxp.c")
        .file("./extern/src/rxpv.c")
        .file("./extern/src/rxr.c")
        .file("./extern/src/ry.c")
        .file("./extern/src/rz.c")
        .file("./extern/src/s00.c")
        .file("./extern/src/s00a.c")
        .file("./extern/src/s00b.c")
        .file("./extern/src/s06.c")
        .file("./extern/src/s06a.c")
        .file("./extern/src/s2c.c")
        .file("./extern/src/s2p.c")
        .file("./extern/src/s2pv.c")
        .file("./extern/src/s2xpv.c")
        .file("./extern/src/sepp.c")
        .file("./extern/src/seps.c")
        .file("./extern/src/sp00.c")
        .file("./extern/src/starpm.c")
        .file("./extern/src/starpv.c")
        .file("./extern/src/sxp.c")
        .file("./extern/src/sxpv.c")
        .file("./extern/src/taitt.c")
        .file("./extern/src/taiut1.c")
        .file("./extern/src/taiutc.c")
        .file("./extern/src/tcbtdb.c")
        .file("./extern/src/tcgtt.c")
        .file("./extern/src/tdbtcb.c")
        .file("./extern/src/tdbtt.c")
        .file("./extern/src/tf2a.c")
        .file("./extern/src/tf2d.c")
        .file("./extern/src/tpors.c")
        .file("./extern/src/tporv.c")
        .file("./extern/src/tpsts.c")
        .file("./extern/src/tpstv.c")
        .file("./extern/src/tpxes.c")
        .file("./extern/src/tpxev.c")
        .file("./extern/src/tr.c")
        .file("./extern/src/trxp.c")
        .file("./extern/src/trxpv.c")
        .file("./extern/src/tttai.c")
        .file("./extern/src/tttcg.c")
        .file("./extern/src/tttdb.c")
        .file("./extern/src/ttut1.c")
        .file("./extern/src/ut1tai.c")
        .file("./extern/src/ut1tt.c")
        .file("./extern/src/ut1utc.c")
        .file("./extern/src/utctai.c")
        .file("./extern/src/utcut1.c")
        .file("./extern/src/xy06.c")
        .file("./extern/src/xys00a.c")
        .file("./extern/src/xys00b.c")
        .file("./extern/src/xys06a.c")
        .file("./extern/src/zp.c")
        .file("./extern/src/zpv.c")
        .file("./extern/src/zr.c")
        .compile("sofa");
}