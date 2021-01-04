from manage.component import *


components = [
    ("core", Cmake("core")),
    ("vecmat", Cargo("vecmat", features=["rand", "approx"])),
    ("dyntype", Cargo("dyntype")),
]
