


`find . -type f -name "*.php" ! -path "*vendor*" ! -path "*node_modules*" ! -path "*.idea*" -exec sh -c 'echo -n "{}: "; git log --format=%H --reverse -- {} | head -n 1' \;`

real    1m1.464s

`fhf`

real    0m37.531s

just walkdir (single threaded)
real    0m3.903s

exclude vendor, node_module, .idea, var/cache, just printing out paths of all php files
real    0m0.099s

recap: find . -type f -name "*.php"
real    0m1.332s

`fhf after excludes`

real    0m32.647s

parallel 100

real    0m24.437s

parallel 8
real    0m33.435s

parallel 200
real    0m12.829s

parallel 1000
real    0m29.025s

parallel 500
real    0m11.961s

vor sortieren:
real    0m12.634s

nach sortieren
real    0m14.086s