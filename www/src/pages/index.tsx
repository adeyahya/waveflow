import React, { useEffect } from "react";
import { Link } from "react-router-dom";
import {
  List,
  ListItem,
  Text,
  Code,
  Box,
  Stack,
  Button,
} from "@chakra-ui/react";
import { useMount } from "react-use";
import useWorkflowStore from "~store/workflows";
import useUserStore from "~store/user";
import useWorkflowHistoryStore from "~store/workflowHistory";
import { useHistory } from "react-router";

const RootPage = () => {
  const workflow = useWorkflowStore();
  const workflowHistory = useWorkflowHistoryStore();
  const user = useUserStore();
  const history = useHistory();

  console.log(workflowHistory.items);

  useMount(async () => {
    workflowHistory.get("1");
    workflow.get();
  });

  useEffect(() => {
    if (!user.loading && !user.email) {
      history.replace("/login");
    }
  }, [user]);

  return (
    <Stack direction="row">
      <Box
        p=".5rem"
        flex={1}
        bg="white"
        borderColor="gray.300"
        borderWidth="1px"
        borderRadius="md"
      >
        <List spacing="1rem">
          {workflow.items.length === 0 && (
            <Box>
              <Text mb=".5rem">No Workflow yet</Text>
              <Button
                onClick={() => {
                  history.push("/workflow/new");
                }}
                size="sm"
              >
                Create One
              </Button>
            </Box>
          )}
          {workflow.items.map((wf) => (
            <ListItem key={wf.slug}>
              <Link to={`/workflow/${wf.slug}`}>
                <Text>{wf.name}</Text>
                <Code>
                  <Text>{wf.content}</Text>
                </Code>
              </Link>
            </ListItem>
          ))}
        </List>
      </Box>
      <Box w="400px"></Box>
    </Stack>
  );
};

export default RootPage;
