import React, { useEffect } from "react";
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
import useUserStore from "~store/user";
import { useHistory } from "react-router";

const Login = () => {
  const user = useUserStore();
  const history = useHistory();
  const { control, handleSubmit } = useLoginForm();
  const submitHandler = handleSubmit(async (data) => {
    await user.auth(data);
  });

  useEffect(() => {
    if (!user.loading && user.username) {
      history.replace("/");
    }
  }, [user]);

  return (
    <Container maxW="5xl">
      <Stack maxW="450px" mx="auto" mt="4rem" mb="2rem">
        <Heading>Login Page</Heading>
        <Text>Lorem ipsum dolor sit, amet consectetur adipisicing elit.</Text>
      </Stack>
      <Stack maxW="450px" mx="auto" spacing="1.5rem">
        <Controller
          name="username"
          control={control}
          rules={{ required: true }}
          render={({ field, fieldState }) => (
            <FormControl isInvalid={fieldState.invalid}>
              <FormLabel>Username</FormLabel>
              <Input
                autoComplete="off"
                onChange={(e) => field.onChange(e.target.value)}
                onBlur={field.onBlur}
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
          <Button flex="1" colorScheme="blue" onClick={submitHandler}>
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
