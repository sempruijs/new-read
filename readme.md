# new-read

changing the way we write so we can focus on making a better world.
It is mostely an experiment than a real product. This repo provides:

- rules for the first version
- old-read new-read convertor written in rust (WIP)


## status

new-read has currently only development for the dutch version, which is in the very pre-alpha fase. nothing is tested.


# nrc

new-read converter (nrc) can convert plain-text files. For example:

```shell
nrc -f hello.md -o hello-new-read.md
```

## installing nrc

### nix

add nrc by adding the following input to your flake:

```nix
nrc.url = "github:sempruijs/new-read";
```

and then adding the input in home.packages

```nix
inputs.nrc.packages.${pkgs.system}.default
```

You can always take a peak at [my nixos config](https://github.com/sempruijs/nixos-config) ;)