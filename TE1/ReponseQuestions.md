# TE1 - Réponse questions
## Questions générales
> 1)

> 2) Le lab 1 portait sur deux attaques; les CSRF et une autre attaque. Quel outil de sécurité est typiquement utilisé pour automatiser l’exploitation de cette autre attaque ? (1 pt)

On utilise SQLmap

> 3) Vous êtes un chercheur en sécurité et vous avez découvert la vulnérabilité logicielle suivante: si on envoie à l’utilisateur ciblé un email, dans lequel on lui demande d’ouvrir un shell et d’exécuter la commande rm -rf /*, le shell déclenche une cascade d’actions dont le résultat final est de détruire le système de fichiers de la machine ciblée. Sous quelle CWE peut-on classer cette vulnérabilité ? Calculez son score CVSS. (2 pts)

Cela peut être une CWE-78 car on fait une injection de commande système en passant par un email qui va demander à l'utilisateur d'executer cette commande.

Score CVSS: CVSS:3.1/C:H/AV:N/A:H/C:H/I:H/PR:L/S:U/UI:R

Donc un score Overall de 7.5 dans le calculateur CVSS 3.1

> 4) Quelle vulnérabilité ayant un impact sur l’intégrité est présente dans le fragment de code suivant ? (2 pts)
> ```
> public void write_data(String filePath) throws IOException {
>    try {
>         File file = new File("", filePath);
>         if (file.exists()) {
>              throw new IOException("file exists");
>         }
>         FileOutputStream fs = new FileOutputStream(file);
>         fs.write(get_my_arbitrary_data().getBytes());
>         fs.close();
>    } catch (IOException e) {
>         System.out.println("Exception: " + e.getMessage());
>    }
> }
> ```



> 5) Voyez-vous d’autres problèmes avec ce code ? (2 pts)
 
 

> 6) Donnez une expression régulière avec laquelle on peut filtrer du HTML fourni par un utilisateur, pour éliminer le risque d’une attaque XSS (2 pts)

 
 
> 7) Qu’est-ce qui est le plus pratique à utiliser, execve() ou system() ? (2 pts)




