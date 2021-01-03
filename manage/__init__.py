from manage.component import *


components = [
    ("rstd", Cmake("rstd", test="rstd_test")),
    ("core", Cmake("core", test="kernel_test")),
    ("vecmat", Cargo("vecmat")),
    ("dyntype", Cargo("dyntype")),
]
