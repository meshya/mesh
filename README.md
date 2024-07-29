# meah
Downloadanager tool with focus on:
1. User experience
2. multi connection download
3. multi source
4. package managers compalibility

## repos
repos located in `/etc/mesh.d/`
```
#/etc/mesh.d/arclinux.conf
[core]
url1
url2
url3
...

[extra]
url1
url2
url3
...

```

## use mesh
```
mesh mesh://core/x64_86/linux.....zst -p pacman -j 8
```


## Develop docs
I write develop progress, Ideas, solutions and code structure and etc at the same time as developing mesh in docs dir from root of repo
