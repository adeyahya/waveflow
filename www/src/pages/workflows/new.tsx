import React, { useRef } from "react";
import { useMount } from "react-use";
import * as monaco from "monaco-editor";
import {
  Box,
  FormControl,
  FormLabel,
  Input,
  Stack,
  Button,
} from "@chakra-ui/react";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
import { useWorkflowForm, Controller } from "../../hooks/forms";
import axios from "axios";

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
  const { control, handleSubmit } = useWorkflowForm();
  let editor: monaco.editor.IStandaloneCodeEditor | null = null;

  useMount(() => {
    if (editorRef.current) {
      editor = monaco.editor.create(editorRef.current, {
        value: "# your code go here",
        language: "shell",
        fontSize: 16,
        roundedSelection: false,
        scrollBeyondLastLine: false,
        readOnly: false,
        theme: "vs-light",
        padding: {
          top: 20,
          bottom: 20,
        },
      });
    }
  });

  const submitHandler = handleSubmit(async (data) => {
    await axios.post("/workflows", {
      name: data.name,
      slug: data.slug,
      content: editor?.getValue(),
    });
  });

  return (
    <Box p="4rem" h="100vh" w="100vw" overflow="auto">
      <Stack direction="column" spacing="1rem">
        <Controller
          control={control}
          name="name"
          rules={{ required: true }}
          render={({ field, fieldState }) => (
            <FormControl isInvalid={fieldState.invalid} isRequired>
              <FormLabel>Name</FormLabel>
              <Input
                onChange={(e) => {
                  field.onChange(e.target.value);
                }}
                onBlur={field.onBlur}
                placeholder="workflow name"
              />
            </FormControl>
          )}
        />
        <Controller
          control={control}
          name="slug"
          rules={{ required: true }}
          render={({ field, fieldState }) => (
            <FormControl isInvalid={fieldState.invalid} isRequired>
              <FormLabel>Slug</FormLabel>
              <Input
                onChange={(e) => {
                  field.onChange(e.target.value);
                }}
                onBlur={field.onBlur}
                placeholder="unique identifier"
              />
            </FormControl>
          )}
        />
        <Box minH="50vh" w="100%" ref={editorRef}></Box>
        <Stack>
          <Button onClick={submitHandler} colorScheme="blue">
            Save Workflow
          </Button>
        </Stack>
      </Stack>
    </Box>
  );
};

export default NewWorkflow;
