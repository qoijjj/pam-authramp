## Threat Model Description

### Overview:

This Pluggable Authentication Module (PAM) is designed to enhance security on personal devices by implementing a dynamic authentication delay mechanism following consecutive failed login attempts. The primary goal is to defend against brute force attacks while avoiding the drawbacks associated with traditional account lockouts.

### Threats Addressed:

1. **Brute Force Attacks:**
   - Adversaries attempting to gain unauthorized access by systematically trying a large number of password combinations.

2. **Denial of Use Attacks:**
   - Attacks aimed at disrupting service availability by triggering account lockouts, commonly exploited in systems with fixed lockout policies.

### Rationale:

1. **User Account Protection:**
   - The module aims to protect user accounts from unauthorized access without resorting to permanent or easily exploitable account lockouts. Instead, it progressively increases authentication delays, making brute force attacks significantly more time-consuming.

2. **Avoiding Denial of Use Risks:**
   - Unlike traditional account lockouts, which can create a denial of use attack surface, the dynamic delay mechanism does not completely prevent access. Instead, it introduces a time-based hurdle for attackers, ensuring legitimate users can eventually regain access.

### Considerations:

1. **Personal Devices:**
   - Tailored for personal devices where users may not have the support of an organizational administrator to unlock accounts.

2. **Aggressive Delay Ramping:**
   - The module employs an aggressive delay escalation strategy to discourage brute force attacks while allowing legitimate users access with increasing delays.

3. **Usability:**
   - Balancing security and usability, the module aims to provide a robust defense without inconveniencing genuine users under normal circumstances.

### Usage Recommendations:

1. **Customization:**
   - Adjust the delay parameters according to your specific security requirements and user tolerance for delays.

2. **Monitoring:**
   - Regularly monitor logs for authentication attempts and adjust delay settings based on observed threat patterns.

3. **Integration with Existing Security Measures:**
   - Consider incorporating this module as part of a layered security approach, complementing existing security measures.

### Limitations:

1. **Not a Silver Bullet:**
   - While effective against brute force attacks, the module should be part of a comprehensive security strategy that includes strong password policies and other security measures.

2. **Limited Data Protection:**
   - On Full Disk Encryption (FDE) devices, PAM authentication takes place in an already unlocked state. As a result, the module primarily protects account access and permissions rather than the data itself. Users employing FDE should consider additional measures to safeguard sensitive data on the device.