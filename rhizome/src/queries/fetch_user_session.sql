SELECT * FROM user
JOIN session ON session.user_id = user.user_id
WHERE session.token = ?;
