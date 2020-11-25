<template>
  <div class="about">
    <div class="content">
      <h1>Introduction</h1>
      
      <p>Ce document constitue l’ébauche d’un outil destiné à répertorier ce que l’on peut considérer comme des gestes culturels.</p>
      
      <p>Les gestes que nous qualifions ici de « culturels » sont des mouvements produits simultanément à la parole ou pouvant parfois être compris sans la parole. Ils sont ancrés culturellement puisqu’ils ne sont pas universels, ou bien leur signification n’est pas universelle.</p>
      
      <p>Notons d’ailleurs que les gestes recensés ici résultent d’une étude sur des populations étudiantes (Université de Rennes II, Université de Nantes, Universiadad Pedagogica Nacional de Bogota) et qu’ils ne peuvent alors être considérés comme représentatifs de l’ensemble de la population française ou colombienne.</p>
      
      <div class="credits">
        <div class="left">
          <br/>
          <h1>Crédits</h1>
          <p>Hélène Olivon, 2017</p>
          <p>Illustrations : Juliette Chautemps</p>
          <p>Modèle photo : Caroline Barbot</p>
          <p>Site web : Adrien Olivon</p>
          <br/>
          <br/>
          <br/>
          <br/>
          <h2><router-link to="/">🢐 Retour</router-link></h2>
        </div>
        <div class="right">
          <img src="logobis.png">
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.about {
  margin: 0 auto;
  max-width: 900px;
  background-image: url(../assets/dot_background.png);
  background-repeat: repeat-y;
  background-position: right;
  height: 100vh;
}

.content {
  padding: 70px;
}

.credits {
  display: flex;
  width: 100%;
}

.credits .left {
  width: 50%;
}

.credits .right {
  width: 50%;
}

.credits .right img {
  height: 450px;
  margin: 0 auto;
}
</style>