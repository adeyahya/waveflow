import React, { useRef } from "react";
import { useMount } from "react-use";
import * as monaco from "monaco-editor";
import { Box } from "@chakra-ui/react";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

// @ts-ignore
self.MonacoEnvironment = {
  getWorker(_: any, label: any) {
    if (label === "typescript" || label === "javascript") {
      return new tsWorker();
    }
    return new editorWorker();
  },
};

const NewWorkflow = () => {
  const editorRef = useRef<HTMLDivElement>(null);

  useMount(() => {
    if (editorRef.current) {
      monaco.editor.create(editorRef.current, {
        value: "# your code go here",
        language: "shell",
        fontSize: 16,
        roundedSelection: false,
        scrollBeyondLastLine: false,
        readOnly: false,
        theme: "vs-dark",
        padding: {
          top: 20,
          bottom: 20,
        },
      });
    }
  });

  return (
    <Box p="4rem" h="100vh" w="100vw" overflow="auto">
      <Box h="100%" w="100%" ref={editorRef}></Box>
    </Box>
  );
};

export default NewWorkflow;
