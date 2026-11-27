//! Batch 1 of the LiteraLua blocks: JSON, IsStudio, IsDescendantOf /
//! IsAncestorOf, GetFullName, Kick, group/friend checks, HSV color, and the
//! ChildAdded / ChildRemoved / Humanoid.Died events. Each Lua snippet goes to
//! blocks and back to Lua, and must come back as the same call.

use visual_source_rs::{encoder, transpile, VisualSource};

fn back(src: &str) -> String {
    let out = transpile::transpile(src, "batch1", true)
        .unwrap_or_else(|e| panic!("transpile failed: {}", e));
    let payload = encoder::encode_payload(&out.report).unwrap();
    let report =
        VisualSource::decode(&payload).unwrap_or_else(|e| panic!("re-decode failed: {}", e));
    visual_source_rs::transpile::detranspile::detranspile(&report).luau
}

fn check(src: &str, needle: &str) {
    let luau = back(src);
    assert!(luau.contains(needle), "missing {:?} in:\n{}", needle, luau);
    assert!(!luau.contains("unsupported block"), "unsupported block in:\n{}", luau);
}

#[test]
fn json_encode_decode() {
    check(
        "local http = game:GetService(\"HttpService\")\nlocal s = http:JSONEncode({a = 1})\nprint(s)",
        "JSONEncode(",
    );
    check(
        "local http = game:GetService(\"HttpService\")\nlocal t = http:JSONDecode(\"{}\")\nprint(t)",
        "JSONDecode(",
    );
}

#[test]
fn is_studio() {
    check("local rs = game:GetService(\"RunService\")\nprint(rs:IsStudio())", "IsStudio()");
}

#[test]
fn descendant_and_ancestor() {
    check("print(workspace.Part:IsDescendantOf(workspace))", "IsDescendantOf(");
    check("print(workspace:IsAncestorOf(workspace.Part))", "IsAncestorOf(");
}

#[test]
fn full_name() {
    check("print(workspace.Part:GetFullName())", "GetFullName()");
}

#[test]
fn kick() {
    check("local p = game.Players.LocalPlayer\np:Kick(\"bye\")", ":Kick(");
}

#[test]
fn group_and_friends() {
    check("local p = game.Players.LocalPlayer\nprint(p:IsInGroup(123))", "IsInGroup(123)");
    check("local p = game.Players.LocalPlayer\nprint(p:GetRankInGroup(123))", "GetRankInGroup(123)");
    check("local p = game.Players.LocalPlayer\nprint(p:IsFriendsWith(5))", "IsFriendsWith(5)");
}

#[test]
fn hsv_color() {
    check("local c = Color3.fromHSV(0.5, 1, 1)\nprint(c)", "Color3.fromHSV(");
}

#[test]
fn child_events() {
    check("workspace.ChildAdded:Connect(function(c)\n  print(c)\nend)", "ChildAdded:Connect(");
    check("workspace.ChildRemoved:Connect(function(c)\n  print(c)\nend)", "ChildRemoved:Connect(");
}

#[test]
fn humanoid_died() {
    check(
        "local h = workspace.Dummy.Humanoid\nh.Died:Connect(function()\n  print(\"died\")\nend)",
        "Died:Connect(",
    );
}
