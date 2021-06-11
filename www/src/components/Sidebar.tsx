import { Box, BoxProps } from "@chakra-ui/react";
import React from "react";

const Sidebar: React.FC<BoxProps> = (props) => {
  return (
    <Box bg="gray.400" px="1rem" minW="250px" {...props}>
      Side bar goes here
    </Box>
  );
};

export default Sidebar;
