// pub mod util;
pub mod dct2d;
pub mod algorithm;
pub mod util;
pub mod yiq;

use std::path::PathBuf;

pub fn do_thing(image_path: &PathBuf) {
    let orig_image =
        image::open(&image_path).expect(&format!("could not load image at {:?}", image_path));

    let mut watermarker = algorithm::Watermarker::new(orig_image);

    let mark = algorithm::Mark::from(&[
        1.5662269184308768,
        -0.139376843912537,
        1.2815220015684436,
        -0.20421716511630486,
        -1.9151579043073244,
        -0.658166893779025,
        -0.690438359551056,
        -0.9039390840485201,
        -1.430590861256953,
        0.17403489666463817,
        1.1843867437984206,
        -0.18854014660283705,
        -0.2865427415427998,
        -1.1291396191516556,
        1.8276179196813933,
        -1.4559209966966444,
        0.06063634411188765,
        0.39770952432264073,
        -0.1324959012950368,
        0.5307678391926361,
        -0.9357662530746624,
        -0.6757437428750647,
        -1.0328916592227266,
        -2.5929875940295117,
        1.3214522099423933,
        1.0789936813157004,
        -1.0970500626908726,
        -1.0149363459927825,
        -0.8163633793092601,
        0.1935138060357104,
        -0.8645177749088816,
        0.36649481944955337,
        0.5480431677522288,
        -0.6744035755481275,
        -0.7335861092675501,
        -0.8495675832547099,
        1.4525188638266644,
        0.3984703422149117,
        1.1880547614476318,
        -0.22026868818196024,
        -1.3983836651712436,
        -0.335941359684234,
        -0.5298091965727759,
        -1.3461041095587585,
        -0.6253987771716789,
        -1.9563295932450864,
        1.5535555663594747,
        -0.26596535194501525,
        -0.8475722263682639,
        -0.3398514239857582,
        0.36710239671806927,
        -0.44404030575664355,
        -2.1584534188151245,
        1.2815956015138104,
        -0.8921417082588244,
        -0.35438491561107555,
        0.5617474988328942,
        -1.784957594386399,
        1.1377392569861078,
        -0.4358390532467968,
        1.3385201250602041,
        0.3362326027790365,
        0.123977493863593,
        2.4498489582622276,
        -0.8764745248194864,
        -0.17084597624830872,
        0.9194389638921413,
        -2.5926131747143417,
        -0.6302676395109199,
        -0.7836157669316295,
        1.1916879247087415,
        1.1298442380891558,
        -1.7287832626482735,
        -0.057659987543850266,
        -0.9510708477111675,
        0.042842940295037095,
        1.4416507341598102,
        -0.37593868816954873,
        1.3212269526434945,
        0.9004664139132619,
        0.26147007994991767,
        -0.8424292598760691,
        1.0352217637525876,
        -0.6958236373918887,
        -1.021178168483524,
        0.20942844128986765,
        -1.2442372371149288,
        0.42083394324833845,
        -0.7240441218993773,
        -0.4989210221530343,
        1.728712862917051,
        0.4011948039920017,
        -1.844856222402798,
        -1.0286438489258478,
        0.3668257943542009,
        0.29546924399429625,
        -1.1514603965859127,
        -0.031564541990516434,
        0.5224353745484088,
        2.7456791406097314,
    ]);
    let config = algorithm::EmbedConfig::default();
    watermarker.mark(config, &[mark]);
    let res = watermarker.result();

    let img_back_to_rgb = res.into_rgb8();
    img_back_to_rgb
        .save(&PathBuf::from("/tmp/watermarked.png"))
        .expect("may not fail");
}
