#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_comment() {
        let comment_json = r#"{"approvals":[{"description":"Verified","type":"Verified","value":"0"},{"description":"Code-Review","oldValue":"0","type":"Code-Review","value":"-1"},{"description":"Workflow","type":"Workflow","value":"0"}],"author":{"email":"test@example.com","name":"Test User","username":"test1"},"change":{"branch":"master","commitMessage":"Let's do stuff\n\nChange-Id: Imaginary\n","id":"123","number":"123","owner":{"email":"test2@example.com","name":"User 2","username":"test2"},"project":"openstack/thing","status":"NEW","subject":"Thing","topic":"topic-thing","url":"https://review.openstack.org/1234567"},"changeKey":{"id":"123"},"comment":"Patch Set 2: Code-Review-1\n\n(1 comment)","eventCreatedOn":1551983427,"patchSet":{"author":{"email":"test@example.com","name":"Test User","username":"test1"},"createdOn":1551365115,"isDraft":false,"kind":"REWORK","number":"2","parents":["123"],"ref":"refs/changes/47/123/2","revision":"fdd011e3c70ad49f4daf84281458165798ce3eb3","sizeDeletions":0,"sizeInsertions":9,"uploader":{"email":"test@example.com","name":"Test User","username":"test1"}},"project":"openstack/thing","refName":"refs/heads/master","type":"comment-added"}"#;
        let comment: GerritMessage = serde_json::from_str(comment_json).unwrap();
        match comment {
            GerritMessage::CommentAdded(comment) => {
                assert_eq!(comment.author.username, "test1");
            }
            _ => unreachable!()
        }

    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub email: Option<String>,
    pub name: Option<String>,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Approval {
    pub description: String,
    pub value: String,
    #[serde(rename="type")]
    pub approval_type: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub branch: String,
    pub commit_message: String,
    pub id: String,
    pub number: String,
    pub owner: User,
    pub project: String,
    pub status: String,
    pub subject: String,
    pub topic: Option<String>,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangeKey {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Patchset {
    pub author: User,
    pub created_on: usize,
    pub is_draft: bool,
    pub kind: String,
    pub number: String,
    pub parents: Vec<String>,
    #[serde(rename="ref")]
    pub commit_ref: String,
    pub revision: String,
    pub size_deletions: isize,
    pub size_insertions: isize,
    pub uploader: User,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMerged {
    pub submitter: User,
    pub new_rev: String,
    pub patch_set: Patchset,
    pub change: Change,
    pub project: String,
    pub ref_name: String,
    pub change_key: ChangeKey,
    pub event_created_on: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentAdded {
    pub approvals: Vec<Approval>,
    pub author: User,
    pub change: Change,
    pub change_key: ChangeKey,
    pub comment: String,
    pub event_created_on: usize,
    pub patch_set: Patchset,
    pub project: String,
    pub ref_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewerAdded;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAbandoned {
    pub abandoner: User,
    pub patch_set: Patchset,
    pub change: Change,
    pub project: String,
    pub ref_name: String,
    pub change_key: ChangeKey,
    pub event_created_on: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefUpdate {
    pub old_rev: String,
    pub new_rev: String,
    pub ref_name: String,
    pub project: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefUpdated {
    pub submitter: Option<User>,
    pub ref_update: RefUpdate,
    pub event_created_on: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefReplicated {
    pub project: String,
    #[serde(rename="ref")]
    pub commit_ref: String,
    pub target_node: String,
    pub status: String,
    pub ref_status: Option<String>,
    pub event_created_on: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefReplicationDone {
    pub project: String,
    #[serde(rename="ref")]
    pub commit_ref: String,
    pub nodes_count: usize,
    pub event_created_on: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefStatus {
    pub project: String,
    #[serde(rename="ref")]
    pub commit_ref: String,
    pub target_node: String,
    pub status: String,
    pub event_created_on: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchsetCreated {
    pub uploader: User,
    pub patch_set: Patchset,
    pub change: Change,
    pub project: String,
    pub ref_name: String,
    pub change_key: ChangeKey,
    pub event_created_on: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum GerritMessage {
    #[serde(rename="change-merged")]
    ChangeMerged(ChangeMerged),
    #[serde(rename="change-abandoned")]
    ChangeAbandoned(ChangeAbandoned),
    #[serde(rename="comment-added")]
    CommentAdded(CommentAdded),
    // #[serde(rename="reviewer-added")]
    // ReviewerAdded(ReviewerAdded),
    #[serde(rename="patchset-created")]
    PatchsetCreated(PatchsetCreated),
    #[serde(rename="ref-updated")]
    RefUpdated(RefUpdated),
    #[serde(rename="ref-replicated")]
    RefReplicated(RefReplicated),
    #[serde(rename="ref-replication-done")]
    RefReplicationDone(RefReplicationDone),
    #[serde(rename="ref-status")]
    RefStatus(RefStatus),
}