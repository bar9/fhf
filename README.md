`find . -type f -name "*.php" ! -path "*vendor*" -exec sh -c 'echo -n "{}: "; git log --format=%H --reverse -- {} | head -n 1' \;`

real    1m1.464s
user    0m33.680s
sys     0m24.914s

`find fhf`

real    0m37.531s
user    0m30.256s
sys     0m7.061s