document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('span.inline-equation svg.typst-doc').forEach(svg => {
        svg.removeAttribute('height');
        svg.removeAttribute('width');
        svg.style.height = '0.65em';
        svg.style.width = 'auto';
    });
});

document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('p.block-equation svg.typst-doc').forEach(svg => {
        svg.removeAttribute('height');
        svg.removeAttribute('width');

        viewBox = svg.viewBox.baseVal;
        viewBoxHeight = viewBox.height;

        blockHeight = String(viewBox.height * 0.08) + 'rem';

        svg.style.height = blockHeight;
        svg.style.width = 'auto';
    });
});
