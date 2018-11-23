export default import("./plasma").then(plasma => {
  const document = self.document;

  Array.from(document.querySelectorAll("plasma.autostart")).forEach((element: HTMLElement) => {
     let minSteps: number = parseInt(element.dataset.minSteps) || plasma.DefaultPlasmaGeneratorOptions.minSteps;
     let maxSteps: number = parseInt(element.dataset.maxSteps) || plasma.DefaultPlasmaGeneratorOptions.maxSteps;
     let width: number = parseInt(element.dataset.plasmaWidth) || plasma.DefaultPlasmaGeneratorOptions.width;
     let height: number = parseInt(element.dataset.plasmaHeight) || plasma.DefaultPlasmaGeneratorOptions.height;
     let targetWidth: number = parseInt(element.dataset.targetWidth) || plasma.DefaultCanvasRendererOptions.width;
     let targetHeight: number = parseInt(element.dataset.targetHeight) || plasma.DefaultCanvasRendererOptions.height;
     if (minSteps >= maxSteps) maxSteps = minSteps + 1;
     if (targetWidth < width) targetWidth = width;
     if (targetHeight < height) targetHeight = height;
     let canvas = document.createElement("canvas");
     element.appendChild(canvas);
     let generator = new plasma.PlasmaGenerator({ width, height, minSteps, maxSteps });
     let renderer = new plasma.CanvasRenderer(canvas, { width: targetWidth, height: targetHeight });
     resize();
     renderer.attach(generator);
     renderer.animate(true);

     self.addEventListener('resize', resize, false);

     function resize() {
         canvas.width = element.offsetWidth || targetWidth;
         canvas.height = element.offsetHeight || targetHeight;
         renderer.ctx.imageSmoothingEnabled = false;
     }
  });

  return plasma;
})
.catch(e => console.error(e));
