MAIN_PAGE ::= """
<html>
  <head>
    <title>Oberled</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    $STYLE
  </head>
  <body>
  <h1>OBERLED</h1>
  <div id="container">
      <a href="/off">Off</a>
      <a href="/bit">Bit</a>
      <a href="/ants">Ants</a>
      <a href="/blink">Blink</a>
      <a href="/cells">Cells</a>
      <a href="/chase">Chase</a>
      <a href="/debug">Debug</a>
      <a href="/holey">Holey</a>
      <a href="/ogpulse">Pulse (OG)</a>
      <a href="/ogstripe">Stripe (OG)</a>
      <a href="/picture">Picture</a>
      <a href="/monalisa">Mona</a>
      <a href="/pulse">Pulse</a>
      <a href="/sockey">Sockey</a>
      <a href="/wave">Wave</a>
      <a href="/dvd">DVD</a>
  </div>
  </body>
</html>
"""

NOT_FOUND_PAGE ::= """
<html>
  <head>
    <title>Oberled - 404</title>
  </head>
  <body>
    <p>Mode not found</p>
  </body>
</html>
"""

STYLE ::= """
<style>
@import url('https://fonts.googleapis.com/css2?family=Red+Hat+Display:wght@300&family=Red+Hat+Mono:wght@300&display=swap');
body {
    background-color: #212121;
    padding: 20px;
}

h1 {
    font-family: 'Red Hat Display', serif;
    font-weight: 700;
    font-size: 2rem;
    color: #fff;
    text-align: center;
}

#container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
    gap: 30px;
    justify-content: center;
}

a {
    font-family: 'Red Hat Mono', monospace;
    font-size: 1.2rem;
    color: #fff;
    aspect-ratio: 1 / 1;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px solid #fff;
    border-radius: 8px;
    text-align: center;
    padding: 2px;
    justify-self: center;
}

a:hover {
    background-color: #353535;
}
</style>
"""