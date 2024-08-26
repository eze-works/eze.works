+++
[
    title: "Optimizing HTML images as a post-processing step",
    date: ~D(2024-03-30),
    labels: ["html"]
]
+++
I have been working on a new marketing site for my employer. The design includes a lot of high resolution image assets. To keep the site snappy, it's useful to avoid downloading huge images when not necessary. On a mobile phone for example, the image could be several time smaller without looking different to the human eye. The [standard approach](https://developer.mozilla.org/en-US/docs/Learn/HTML/Multimedia_and_embedding/Responsive_images) to optimize images is to generate multiple versions of the image at different sizes, then use the image `srcset` attribute to tell the browser which version to load. These are called "responsive images".

This is what the markup for a responsive image might look like:
```html
<img
  srcset="elva-fairy-480w.jpg 480w, elva-fairy-800w.jpg 800w"
  sizes="(max-width: 600px) 480px,
         800px"
  src="elva-fairy-800w.jpg"
  alt="Elva dressed as a fairy" />

```
I don't want to manually figure this out whenever I need to insert an image. We are using [Hugo](https://gohugo.io/), so my first thought was to write a [partial](https://gohugo.io/templates/partials/) that did all the work to make a simple `<img>` tag responsive. Like a gift from above, that same day, [jampack was posted to Hacker News](https://news.ycombinator.com/item?id=39816836). [Jampack](https://jampack.divriots.com/) is a CLI tool that takes a static site as input (i.g. a folder with a bunch of HTML, CSS, JS & assets) and optimizes it by doing exactly what I had set out to do and more. This means I can continue to write standard unresponsive `<img>` tags but still get responsive images in the final output of the site. All for the cheap price of four new words added to the build script: 
```bash
npx @divriots/jampack ./public
```

My joy is immense.
