import { Box, BoxProps } from "@chakra-ui/react";
import React from "react";

// @ts-ignore
import Logo from "../resources/logo-with-text.svg?component";

const Sidebar: React.FC<BoxProps> = (props) => {
  return (
    <Box
      pt="1rem"
      bg="white"
      px="1rem"
      minW="250px"
      borderColor="gray.300"
      borderRightWidth="1px"
      {...props}
    >
      <Box w="130px">
        <Logo />
      </Box>
    </Box>
  );
};

export default Sidebar;
