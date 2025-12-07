// Express server test
import express from 'express';

const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello from Crabby + Express! 🦀');
});

const server = app.listen(port, () => {
  console.log(`✅ Express server running on http://localhost:${port}`);
  console.log('Server framework works with Crabby!');
  
  // Close after verification
  setTimeout(() => {
    server.close();
    console.log('✅ Test complete!');
    process.exit(0);
  }, 1000);
});
