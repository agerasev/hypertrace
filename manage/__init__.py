from manage.component import *


components = [
    ("kernel", Cmake("kernel")),
    ("vecmat", Cargo("vecmat", features=["rand", "approx"])),
    ("dyntype", Cargo("dyntype")),
]
