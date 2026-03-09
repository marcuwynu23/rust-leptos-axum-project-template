/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/**/*.rs",
    "./frontend/**/*.rs",
    "./style/**/*.css",
    "./style/tailwind-content-ref.html",
  ],
  // Safelist for class names that only appear in Rust variables (e.g. api.rs).
  safelist: [
    "inline-flex", "items-center", "gap-1.5", "text-zinc-500", "text-emerald-500", "text-red-400",
    "inline-block", "h-2", "w-2", "animate-pulse", "rounded-full", "bg-zinc-500", "bg-emerald-500", "bg-red-400",
  ],
  theme: {
    extend: {
      colors: {
        accent: { DEFAULT: "#a78bfa", hover: "#c4b5fd" },
      },
    },
  },
  plugins: [],
};
