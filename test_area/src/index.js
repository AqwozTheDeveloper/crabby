const express = require('express');

const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello from Crabby Server! 🦀');
});

app.listen(port, () => {
  console.log(`🚀 Server ready at http://localhost:${port}`);
});
