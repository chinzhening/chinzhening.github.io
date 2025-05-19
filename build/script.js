document.addEventListener('DOMContentLoaded', function () {
    document.querySelectorAll('span.inline-equation svg.typst-doc').forEach(function (svg) {
        if (svg instanceof SVGElement) {
            svg.removeAttribute('height');
            svg.removeAttribute('width');
            svg.style.height = '0.65em';
            svg.style.width = 'auto';
        }
    });
    document.querySelectorAll('p.block-equation svg.typst-doc').forEach(function (svg) {
        if (svg instanceof SVGSVGElement) {
            svg.removeAttribute('height');
            svg.removeAttribute('width');
            var viewBox = svg.viewBox.baseVal;
            var viewBoxHeight = viewBox.height;
            var blockHeight = "".concat(viewBoxHeight * 0.08, "rem");
            svg.style.height = blockHeight;
            svg.style.width = 'auto';
        }
    });
});
