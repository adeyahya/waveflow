import { defineConfig } from "vite";
import reactRefresh from "@vitejs/plugin-react-refresh";
import tsconfigPaths from "vite-tsconfig-paths";
import { getAliases } from "vite-aliases";

const aliases = getAliases();

export default defineConfig({
  plugins: [reactRefresh(), tsconfigPaths()],
  resolve: {
    alias: aliases,
  },
});
