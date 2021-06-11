import { Box, BoxProps } from "@chakra-ui/react";
import React from "react";

// @ts-ignore
import Logo from "../resources/waveflow.svg?component";

const Sidebar: React.FC<BoxProps> = (props) => {
  return (
    <Box pt="1rem" bg="gray.200" px="1rem" minW="250px" {...props}>
      <Box w="180px">
        <Logo />
      </Box>
    </Box>
  );
};

export default Sidebar;
