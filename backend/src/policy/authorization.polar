allow(actor, action, resource) if
  has_permission(actor, action, resource);

# Users can see each other.
has_permission(_: User, "read", _: User);

# A User can read their own profile.
has_permission(_: User{id: id}, "read_profile", _: User{id: id});

actor User {}

resource Project {
  roles = ["admin", "maintainer", "reader"];
  permissions = [
    "read",
    "list_documents",
    "create_documents",
    "update_documents",
    "delete_documents",
  ];

  "create_documents" if "admin";
  "update_documents" if "admin";
  "delete_documents" if "admin";


  "read" if "reader";
  "list_documents" if "reader";
}

has_role(user: User, name: String, project: Project) if
    role in user.project_roles and
    role matches { name: name, project: project };

has_relation(org: Org, "parent", project: Project) if project.org = org;

resource Document {
  roles = ["creator"];
  permissions = ["read", "close"];
  relations = { parent: Project };
  "read" if "reader" on "parent";
  "close" if "maintainer" on "parent";
  "close" if "creator";
}

has_relation(project: Project, "parent", document: Document) if document.project = project;

has_role(user: User, "creator", document: Document) if
    document.creator = user;