import React from "react";
import {
  Container,
  Heading,
  Input,
  Text,
  Stack,
  FormControl,
  FormLabel,
  Button,
} from "@chakra-ui/react";

const Login = () => {
  return (
    <Container maxW="5xl">
      <Stack maxW="450px" mx="auto" mt="4rem" mb="2rem">
        <Heading>Login Page</Heading>
        <Text>Lorem ipsum dolor sit, amet consectetur adipisicing elit.</Text>
      </Stack>
      <Stack maxW="450px" mx="auto" spacing="1.5rem">
        <FormControl isRequired>
          <FormLabel>Username</FormLabel>
          <Input type="email" />
        </FormControl>

        <FormControl isRequired>
          <FormLabel>Password</FormLabel>
          <Input type="password" />
        </FormControl>

        <Stack direction="row" justify="space-around">
          <Button flex="1" colorScheme="blue">
            Login
          </Button>
          <Button display="block" px="2rem" variant="ghost">
            Register
          </Button>
        </Stack>
      </Stack>
    </Container>
  );
};

export default Login;
