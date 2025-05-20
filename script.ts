// Dynamically add styling to SVG elements
document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('span.inline-equation svg.typst-doc').forEach(svg => {
        if (svg instanceof SVGElement) {
            // Resizing
            svg.removeAttribute('height');
            svg.removeAttribute('width');
            svg.style.height = '0.65em';
            svg.style.width = 'auto';
            
        }
    });
    document.querySelectorAll('p.block-equation svg.typst-doc').forEach(svg => {
        if (svg instanceof SVGSVGElement) {
            // Resizing
            svg.removeAttribute('height');
            svg.removeAttribute('width');

            const viewBox = svg.viewBox.baseVal;
            const viewBoxHeight = viewBox.height;
            const blockHeight = `${viewBoxHeight * 0.08}rem`;

            svg.style.height = blockHeight;
            svg.style.width = 'auto';

        }
    }); 
});

// Apply default, light, or dark mode
window.addEventListener("load", event => {
    const userPreference = localStorage.getItem("theme");

    if (userPreference) {
        if (userPreference === "dark") {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }
    } else {
        if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }
    }
});

function toggleDarkMode() {
    console.log(`Toggling dark mode`);
    const isDark = document.documentElement.classList.contains("dark");
    document.documentElement.classList.toggle("dark", !isDark);
    localStorage.setItem("theme", isDark ? "light" : "dark");
}