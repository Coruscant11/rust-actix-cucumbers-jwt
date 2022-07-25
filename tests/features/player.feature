Feature: Player

    Scenario: Register player not already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I register the player
        Then the player is succesfully registered
        And I receive a code 200

    Scenario: Register a player already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I register the player
        Then the player is not registered
        And I receive a code 400

    Scenario: Register a player with invalid data
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        And the discord_id is invalid
        When I register the player
        Then the player is not registered
        And I receive a code 400

    Scenario: Get a player not registered yet
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I get the player
        Then I receive a code 404

    Scenario: Get a player already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I get the player
        Then I receive a code 200
        And the player is returned

    Scenario: Get all players
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
            | 0987654321 | Casual | 444555666 | 666554444 |
        And the discord_id is already registered
        When I get all players
        Then I receive a code 200
        And the players are returned

    Scenario: Update a player not registered yet
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I update the player
        Then I receive a code 404

    Scenario: Update a player already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I update the player
        Then I receive a code 200
        And the player is updated

    Scenario: Update a player with invalid data
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        And the discord_id is invalid
        When I update the player
        Then I receive a code 400

    Scenario: Delete a player not registered yet
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I delete the player
        Then I receive a code 404

    Scenario: Delete a player already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I delete the player
        Then I receive a code 200
        And the player is deleted

    Scenario: Delete a player with invalid data
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        And the discord_id is invalid
        When I delete the player
        Then I receive a code 400ÍÍ