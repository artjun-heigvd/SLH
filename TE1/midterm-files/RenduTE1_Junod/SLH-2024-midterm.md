# Questions Générales

## 1
Un token anti-CSRF est surtout efficace s'il est secret et unique, si il a la même valeur que le cookie stocké alors il ne sera pas secret et sera prévisible pour l'attaquant ce qui lui permet de réussir son attaque s'il a l'info de la valeur du ccokie donc assez facilmement.
## 2
On utilise SQLmap
## 3
Cela peut être une CWE-78 car on fait une injection de commande système en passant par un email qui va demander à l'utilisateur d'executer cette commande.

Score CVSS: CVSS:3.1/C:H/AV:N/A:H/C:H/I:H/PR:L/S:U/UI:R

Donc un score Overall de 7.5 dans le calculateur CVSS 3.1
## 4
On écrit dans le fichier ouvert s'il n'existe pas (ligne 4: if(file.exists())) ce qui veu dire que l'on va écrire des données dans un espace mémoire non controlé et donc on ne garantit pas l'intégrité de notre mémoire.

Il faudrait plutot envoyer une exception si le fichier n'existe pas et executer le reste du code s'il existe.
## 5
On ne va pas fermer le fichier s'il y a une exception il faudrait garantir la fermeture du fichier quand on catch une exception afin d'être sur que l'on ne pourra pas accéder à cette emplacement mémoire par la suite.
## 6
Trouvé sur OWASP: ^#?([a-f]|[A-F]|[0-9]){3}(([a-f]|[A-F]|[0-9]){3})?$
## 7
Il faut plutôt utiliser `execve()` car `system()` appelle le shell pour executer la commande, le comportement de l'execution va donc dépendre de l'utilisateur ce qui n'est pas le cas avec `execve()` car il executera juste le program qui lui est passé en argument.
# Revue de code

## 8
On alloue de la mémoire dans les `get_customer()` et `get_dealer()` avec strdup() qui ne sera jamais libérée.

Corr:
utiliser `free()` sur les valeur allouée par strdup() avant de free la struct.
## 9
La fonction main ne vérifie que le nombre d'agument mais pas leur taille ce qui peut entrainer des débordements si l'utilisateur donne une taille trop grande.

Corr:

## 10
On utilise atoi sans vérifier que la conversion se fasse, si on devait utiliser la transaction par la suite on pourrait avoir 0 grammes dans les informations.

Corr:

## 11
Une fois de plus on ne vérifie que le nombre n'argument mais pas que les valeurs passée soit valide l'utilisateur peut donc rentrer ce qu'il veut et on ne pourra pas prévoir ce que le programme fait avec.

Corr:

## 12
malloc et strdup peuvent échouer mais on ne gère pas ces cas.

Corr:

## 13
On augmente les points de fidélités même si la transaction échoue.

Corr:




