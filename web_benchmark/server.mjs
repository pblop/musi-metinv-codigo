import express from 'express';
import path from 'node:path';
import fs from 'node:fs';

const app = express();
const PORT = process.argv[2] || 6010;

app.use((req, res, next) => {
  // Enable COOP and COEP headers for cross-origin isolation
  // This provides better performance.now() precision.
  res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
  res.setHeader('Cross-Origin-Embedder-Policy', 'credentialless');
  next();
});

app.use(express.static(path.join(process.cwd(), 'public')));

const testDir = path.join(process.cwd(), 'public', 'tests');
const testNames = fs.readdirSync(testDir).filter(file => file.endsWith('.json'));

app.get("/testlist", (req, res) => {
  res.json(testNames);
});

const server = app.listen(PORT, () => {
  console.log(`Server is running at http://localhost:${PORT}`);
});
server.on('error', (err) => {
  console.error('Server failed to start:', err);
});
