package kz.ilotterytea.fembajbot.entities;

import jakarta.persistence.*;
import org.hibernate.annotations.CreationTimestamp;
import org.hibernate.annotations.UpdateTimestamp;

import java.util.Date;

/**
 * @author ilotterytea
 * @version 1.0
 */
@Entity
@Table(name = "channels")
public class Channel {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;

    @Column(name = "alias_id", updatable = false, unique = true, nullable = false)
    private Integer aliasId;

    @CreationTimestamp
    @Temporal(TemporalType.TIMESTAMP)
    @Column(name = "created_at", updatable = false, nullable = false)
    private Date creationTimestamp;

    @UpdateTimestamp
    @Temporal(TemporalType.TIMESTAMP)
    @Column(name = "updated_at", nullable = false)
    private Date updateTimestamp;

    @Temporal(TemporalType.TIMESTAMP)
    @Column(name = "opt_outed_at")
    private Date optOutTimestamp;

    public Channel(Integer aliasId) {
        this.aliasId = aliasId;
    }

    public Channel() {}

    public Integer getId() {
        return id;
    }

    public void setId(Integer id) {
        this.id = id;
    }

    public Integer getAliasId() {
        return aliasId;
    }

    public void setAliasId(Integer aliasId) {
        this.aliasId = aliasId;
    }

    public Date getCreationTimestamp() {
        return creationTimestamp;
    }

    public void setCreationTimestamp(Date creationTimestamp) {
        this.creationTimestamp = creationTimestamp;
    }

    public Date getUpdateTimestamp() {
        return updateTimestamp;
    }

    public void setUpdateTimestamp(Date updateTimestamp) {
        this.updateTimestamp = updateTimestamp;
    }

    public Date getOptOutTimestamp() {
        return optOutTimestamp;
    }

    public void setOptOutTimestamp(Date optOutTimestamp) {
        this.optOutTimestamp = optOutTimestamp;
    }
}
