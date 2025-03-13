# Gestion des Branches avec Git

## 1. Voir les branches existantes
```bash
git branch
```
- **Branche actuelle** : Marquée par `*`.
- **Lister les branches distantes** :
  ```bash
  git branch -r
  ```
- **Lister toutes les branches (locales + distantes)** :
  ```bash
  git branch -a
  ```

## 2. Créer une nouvelle branche
```bash
git branch nom-de-branche
```
Ou directement en changeant de branche :
```bash
git checkout -b nom-de-branche
```

## 3. Changer de branche
```bash
git checkout nom-de-branche
```
(Si Git 2.23+ est installé, on peut utiliser :)
```bash
git switch nom-de-branche
```

## 4. Pousser une nouvelle branche sur GitHub
```bash
git push -u origin nom-de-branche
```

## 5. Fusionner une branche dans `main`
1. **Revenir sur `main`** :
   ```bash
   git checkout main
   ```
2. **Mettre à jour `main`** :
   ```bash
   git pull origin main
   ```
3. **Fusionner la branche** :
   ```bash
   git merge nom-de-branche
   ```
4. **Résoudre les conflits (si nécessaire), puis valider (`git commit`)**.
5. **Pousser sur GitHub** :
   ```bash
   git push origin main
   ```

## 6. Supprimer une branche
- **Localement** :
  ```bash
  git branch -d nom-de-branche
  ```
  (Utiliser `-D` si la branche n'est pas encore fusionnée)
- **Sur GitHub** :
  ```bash
  git push origin --delete nom-de-branche
  ```

## 7. Travailler avec des branches distantes
- **Récupérer toutes les branches distantes** :
  ```bash
  git fetch --all
  ```
- **Créer une copie locale d’une branche distante** :
  ```bash
  git checkout -b nom-de-branche origin/nom-de-branche
  ```
- **Mettre à jour une branche locale avec les modifications distantes** :
  ```bash
  git pull origin nom-de-branche
  ```

## 8. Ajouter, Committer et Pousser des modifications
1. **Ajouter les fichiers modifiés ou nouveaux** :
   ```bash
   git add .
   ```
   (Ou spécifier un fichier particulier : `git add mon_fichier.txt`)
2. **Créer un commit avec un message explicite** :
   ```bash
   git commit -m "Description des modifications"
   ```
3. **Envoyer les modifications vers le dépôt distant** :
   ```bash
   git push origin nom-de-branche
   ```

## Exemple de Workflow
1. Créer une branche `feature-ajout-son` :
   ```bash
   git checkout -b feature-ajout-son
   ```
2. Développer, puis ajouter et committer les changements :
   ```bash
   git add .
   git commit -m "Ajout du système de son"
   ```
3. Pousser sur GitHub :
   ```bash
   git push -u origin feature-ajout-son
   ```
4. Faire une **Pull Request** sur GitHub.
5. Une fois validé, **fusionner dans `main`**.
6. Supprimer la branche après la fusion :
   ```bash
   git branch -d feature-ajout-son
   git push origin --delete feature-ajout-son
   ```

