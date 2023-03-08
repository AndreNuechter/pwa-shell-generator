const {
    dest,
    parallel,
    series,
    src,
} = require('gulp');
const express = require('express');
const minifyCSS = require('gulp-csso');
const replace = require('gulp-replace');
const fs = require('fs').promises;
const gulpEsbuild = require('gulp-esbuild');
const htmlMinify = require('html-minifier');
const path = require('path');

const inputDir = 'src';
const outputDir = 'docs';

exports.build = series(cleanOutputDir, parallel(html, css, js, pwaAssets));
exports.dev = dev;

async function cleanOutputDir() {
    emptyDir(outputDir);
}

async function emptyDir(dir) {
    const dirents = await fs.readdir(dir, {
        encoding: 'utf-8',
        withFileTypes: true
    });

    for (const dirent of dirents) {
        const nextStep = path.join(dir, dirent.name);

        if (dirent.isDirectory()) {
            await emptyDir(nextStep);
            fs.rmdir(nextStep);
        } else {
            fs.unlink(nextStep, err => {
                if (err) throw err;
            });
        }
    }
}

function html() {
    return src(`${inputDir}/index.html`)
        .on('data', (file) => {
            file.contents = Buffer.from(htmlMinify.minify(file.contents.toString(), {
                removeComments: true,
                removeEmptyAttributes: true,
                removeRedundantAttributes: true,
                removeScriptTypeAttributes: true,
                removeStyleLinkTypeAttributes: true,
                sortClassName: true,
                useShortDoctype: true,
                collapseWhitespace: true
            }));
            return file.contents;
        })
        .pipe(dest(outputDir));
}

function css() {
    return src(`${inputDir}/index.css`)
        .pipe(minifyCSS())
        .pipe(dest(outputDir));
}

function js() {
    const esbuildOptions = {
        minify: true,
        minifyIdentifiers: true,
        minifySyntax: true,
    };
    src(`${inputDir}/service-worker.js`)
        .pipe(gulpEsbuild({
            outfile: 'service-worker.js',
            ...esbuildOptions
        }))
        .pipe(replace('<APP_VERSION>', Date.now()))
        .pipe(dest(outputDir));
    return src(`${inputDir}/index.js`)
        .pipe(gulpEsbuild({
            outfile: 'index.js',
            bundle: true,
            ...esbuildOptions
        }))
        .pipe(dest(outputDir));
}

function pwaAssets() {
    src(`${inputDir}/images/*.png`)
        .pipe(dest(`${outputDir}/images`));
    return src(`${inputDir}/manifest.json`)
        .pipe(dest(outputDir));
}

function dev() {
    const app = express();
    const port = 3001;
    app.use(express.static(inputDir));
    app.listen(port, () => console.log(`App running @ http://localhost:${port}`));
}