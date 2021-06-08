import React from "react";
import { Link } from "react-router-dom";

const RootPage = () => {
  return (
    <p>
      Lorem ipsum dolor sit amet consectetur, adipisicing elit. Officia sit
      quidem corrupti dicta aut qui quam quis distinctio odio, ab nobis atque
      fugit veritatis illo cupiditate ea esse? Magni, deserunt.
      <Link to="/login">Login</Link>
    </p>
  );
};

export default RootPage;
