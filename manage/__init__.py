from manage.component import *


components = [
    ("rstd", Cmake("rstd", test="rstd_test")),
    ("core", Cmake("core")),
    ("vecmat", Cargo("vecmat")),
    ("dyntype", Cargo("dyntype")),
]
