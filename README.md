![](/img/go-goggles.png)
# lunette-mips-rs

|             Brice              |            code             |
| :----------------------------: | :-------------------------: |
| ![](/img/brice-no-goggles.png) |      ![](/img/hex.png)      |
|  ![](/img/brice-goggles.png)   | ![](/img/mips-assembly.png) |


Lunettes-Mips is a mips disassembler for the mips instruction set, but is not finished yet, doesn't disassemble every instruction and doesn't implement the 64bits version. I will document it when publishing the very first working version.

# Motivation

J'ai voulu faire un projet afin d'avoir quelque chose à présenter le jour où je voudrais rejoindre une école ou trouver un travail en informatique, pour contribuer comme je peux à la communauté de reverse engineering et voir un de mes projets utiliser par des gens pour qu'ils réalisent leurs propres projets ou fassent des outils à partir de mon projet et afin d'apprendre le Rust. J'ai choisi ce nom, car j'aime bien la troisième génération de pokémon, on y trouve les lunettes sable pour pouvoir aller dans le désert pendant des tempêtes de sable.

# Contribution?

Je n'accepte pas de contribution pour le moment mais peut-être dans le futur, mais je suis toujours ouvert au critiques et conseils.

# Goals

- [ ] Rendre la lib cross platform
- [x] Aucune dépendance
- [x] Pas d'allocation de mémoire enfin je crois
- [x] Thread safe(qu'un thread n'est utilisé)
- [ ] Rapide????
- [ ] Lisible T_T
# Inspiration

- [Zydis](https://github.com/zyantific/zydis)

# Aides

- [Online assembler/disassembler](https://yozan233.github.io/Online-Assembler-Disassembler/)
1. Manuel Volume 1
	- https://riteme.site/nscscc/doc/mips/Volume%20I:%20Introduction%20to%20MIPS32%20Architecture.pdf
2. Manuel Volume 2
	- https://riteme.site/nscscc/doc/mips/Volume%20II:%20MIPS32%20Instruction%20Set.pdf