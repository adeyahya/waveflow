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
  FormErrorMessage,
} from "@chakra-ui/react";
import { useLoginForm, Controller } from "~hooks/forms";

const Login = () => {
  const { control, handleSubmit } = useLoginForm();
  const submitHandler = handleSubmit((data) => {
    console.log(data);
  });

  return (
    <Container maxW="5xl">
      <Stack maxW="450px" mx="auto" mt="4rem" mb="2rem">
        <Heading>Login Page</Heading>
        <Text>Lorem ipsum dolor sit, amet consectetur adipisicing elit.</Text>
      </Stack>
      <Stack
        onSubmit={submitHandler}
        as="form"
        maxW="450px"
        mx="auto"
        spacing="1.5rem"
      >
        <Controller
          name="username"
          control={control}
          rules={{ required: true }}
          render={({ field, fieldState }) => (
            <FormControl isInvalid={fieldState.invalid}>
              <FormLabel>Username</FormLabel>
              <Input
                onChange={(e) => field.onChange(e.target.value)}
                onBlur={field.onBlur}
                type="email"
              />
              <FormErrorMessage>
                {fieldState.error?.message || fieldState.error?.type}
              </FormErrorMessage>
            </FormControl>
          )}
        />
        <Controller
          name="password"
          control={control}
          rules={{ required: true }}
          render={({ field, fieldState }) => (
            <FormControl isInvalid={fieldState.invalid}>
              <FormLabel>Password</FormLabel>
              <Input
                onChange={(e) => field.onChange(e.target.value)}
                onBlur={field.onBlur}
                type="password"
              />
              <FormErrorMessage>
                {fieldState.error?.message || fieldState.error?.type}
              </FormErrorMessage>
            </FormControl>
          )}
        />
        <Stack direction="row" justify="space-around">
          <Button
            type="submit"
            onSubmit={submitHandler}
            flex="1"
            colorScheme="blue"
          >
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
