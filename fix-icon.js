const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const iconPath = path.join(__dirname, 'src-tauri', 'icons', 'icon.png');

sharp(iconPath)
  .ensureAlpha()
  .png()
  .toFile(iconPath + '.fixed.png')
  .then(() => {
    fs.renameSync(iconPath + '.fixed.png', iconPath);
    console.log('Icon converted to RGBA');
  })
  .catch(err => {
    console.error('Error:', err);
    process.exit(1);
  });
