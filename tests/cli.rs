use assert_cmd::{prelude::*}; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
#[test]
fn test_main() -> Result<(), Box<dyn std::error::Error>>{
    let mut cmd = Command::cargo_bin("starfield")?;

    cmd.arg("src/sf.map").arg("src/01.danzi.txt").arg("src/jiandao.base.dict.yaml").arg("src/add.dict");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("石氏	ekekvu
恐同	kytyv
韩蛛俐	hqlu
星穹铁道	xqtdo
南梦芽	nmyuv
匹诺康尼	pnkn
冰呪	bgfdo
摘采	qhchi
摘采	fhchi
尼尔	nkxja
彩六	chlqu
女武神	nwea
兆亿	fzyk
兆亿	qzyk
超级碗	wjwva
超级碗	jjwv
霜华	emhq
霜华	exhq
霜雪千年	exqnv"));

    Ok(())
}
