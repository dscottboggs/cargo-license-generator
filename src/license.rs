use derive_license::impl_license;

pub trait License {
    fn notice(&self, year: u32, name: &str, project: &str) -> String;
}

#[impl_license("../files/agpl-3.0.txt", YEAR, AUTHOR, PROJECT)]
pub struct AGPL;

#[impl_license("../files/apache-2.0.txt", YEAR, AUTHOR)]
pub struct Apache;

#[impl_license("../files/bsd-3.0.txt", YEAR, AUTHOR)]
pub struct BSD;

#[impl_license("../files/cc-by-4.0.txt", AUTHOR, PROJECT)]
pub struct CcBy;

#[impl_license("../files/cc-by-nc-4.0.txt", AUTHOR, PROJECT)]
pub struct CcByNc;

#[impl_license("../files/cc-by-nc-sa-4.0.txt", AUTHOR, PROJECT)]
pub struct CcByNcSa;

#[impl_license("../files/cc-by-sa-4.0.txt", AUTHOR, PROJECT)]
pub struct CcBySa;

#[impl_license("../files/cc-zero-1.0.txt", AUTHOR)]
pub struct CCZero;

#[impl_license("../files/gpl-3.0.txt", YEAR, AUTHOR, PROJECT)]
pub struct GPL;

#[impl_license("../files/lgpl-3.0.txt")]
pub struct LGPL;

#[impl_license("../files/mit.txt", YEAR, AUTHOR)]
pub struct Mit;

#[impl_license("../files/mpl-2.0.txt", YEAR, AUTHOR)]
pub struct MPL;

#[impl_license("../files/unlicense.txt")]
pub struct UNLICENSE;
