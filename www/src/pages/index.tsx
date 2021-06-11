import React, { useEffect } from "react";
import { Link } from "react-router-dom";
import { List, ListItem, Text } from "@chakra-ui/react";
import { useMount } from "react-use";
import useWorkflowStore from "~store/workflows";
import useUserStore from "~store/user";
import { useHistory } from "react-router";

const RootPage = () => {
  const workflow = useWorkflowStore();
  const user = useUserStore();
  const history = useHistory();

  useMount(async () => {
    // await user.fetch();
    await workflow.get();
  });

  useEffect(() => {
    if (!user.loading && !user.email) {
      history.replace("/login");
    }
  }, [user]);

  return (
    <List>
      {workflow.items.map((wf) => (
        <ListItem key={wf.slug}>
          <Link to={`/workflows/${wf.slug}`}>
            <Text>{wf.slug}</Text>
          </Link>
        </ListItem>
      ))}
    </List>
  );
};

export default RootPage;
