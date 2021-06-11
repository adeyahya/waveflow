import MonacoEditorNlsPlugin, {
  esbuildPluginMonacoEditorNls,
  Languages,
} from "vite-plugin-monaco-editor-nls";
import { defineConfig } from "vite";
import reactRefresh from "@vitejs/plugin-react-refresh";
import tsconfigPaths from "vite-tsconfig-paths";
import reactSvg from "vite-plugin-react-svg";
import { getAliases } from "vite-aliases";

const aliases = getAliases();

export default defineConfig({
  plugins: [
    reactRefresh(),
    tsconfigPaths(),
    MonacoEditorNlsPlugin(),
    reactSvg(),
  ],
  optimizeDeps: {
    esbuildOptions: {
      plugins: [
        esbuildPluginMonacoEditorNls({
          locale: Languages.id,
        }),
      ],
    },
  },
  resolve: {
    alias: aliases,
  },
});
