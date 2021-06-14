import React from "react";
import { Stack, Box } from "@chakra-ui/react";
import Sidebar from "~components/Sidebar";

const DefaultLayout: React.FC = (props) => {
  return (
    <Stack bg="#f6f8fa" h="100vh" direction="row" justifyContent="stretch">
      <Sidebar />
      <Box
        position="relative"
        p="1rem"
        flex={1}
        maxH="100vh"
        overflowY="scroll"
      >
        {props.children}
      </Box>
    </Stack>
  );
};

export default DefaultLayout;
