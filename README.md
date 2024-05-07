`find . -type f -name "*.php" ! -path "*vendor*" -exec sh -c 'echo -n "{}: "; git log --format=%H --reverse -- {} | head -n 1' \;`

real    1m1.464s
user    0m33.680s
sys     0m24.914s

`fhf`

real    0m37.531s
user    0m30.256s
sys     0m7.061s

just walkdir (single threaded)
real    0m3.903s
user    0m0.495s
sys     0m0.727s

exclude vendor, node_module, .idea, var/cache, just printing out paths of all php files
real    0m0.099s
user    0m0.034s
sys     0m0.066s

recap: find . -type f -name "*.php"
real    0m1.332s
user    0m0.179s
sys     0m0.960s

`fhf after excludes`

real    0m32.647s
user    0m29.422s
sys     0m3.053s

parallel 100

real    0m24.437s
user    0m54.913s
sys     1m29.651s

parallel 8
real    0m33.435s
user    1m6.493s
sys     2m40.626s

parallel 200
real    0m12.829s
user    0m37.925s
sys     0m14.054s

parallel 1000
real    0m29.025s
user    0m29.628s
sys     0m3.212s

parallel 500
real    0m11.961s
user    0m36.179s
sys     0m13.526s