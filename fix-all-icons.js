const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const iconsDir = path.join(__dirname, 'src-tauri', 'icons');
const files = ['icon.png', '32x32.png', '128x128.png', '128x128@2x.png'];

files.forEach((f) => {
  const p = path.join(iconsDir, f);
  if (fs.existsSync(p)) {
    sharp(p)
      .ensureAlpha()
      .png()
      .toFile(p + '.tmp')
      .then(() => {
        fs.renameSync(p + '.tmp', p);
        console.log('fixed', f);
      })
      .catch((err) => {
        console.error('err', f, err);
      });
  }
});
